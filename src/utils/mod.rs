use url::Url;
use WebSocket::{ Result, Error, ErrorKind };
use httparse;
use crypto::sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use chrono::Utc;
use hex::ToHex;
use settings::auth::Authorization;

pub struct HttpData {
  url: Url,
  auth: Authorization
}

impl HttpData {
  pub fn new(path: &str, auth: Authorization) -> Result<Self> {
    match Url::parse(&format!("ws://localhost.com{}", path)) {
      Ok(url) => Ok(HttpData { url: url, auth: auth }),
      Err(e) => Err(Error::new(ErrorKind::Http(httparse::Error::NewLine), format!("Cannot parse request: {}", e)))
    }
  }

  pub fn get_group(&self) -> String {
    let mut group = self.url.path().to_lowercase();

    if group.starts_with("/") {
      group.remove(0);
    }

    let len = match group.len() > 0 {
      true => group.len()- 1,
      _ => 0
    };

    if group.ends_with("/") {
      group.remove(len);
    }

    group
  }

  pub fn validate(&self) -> Option<Error> {

    let (token, public_key) = match self.get_token_and_public_key(
      self.auth.get_token_name().unwrap_or("token".to_string()).as_str(),
      self.auth.get_time_name().unwrap_or("nonce".to_string()).as_str()
      ) {
      Some((t,k)) => (t,k),
      _ => return Some(Error::new(ErrorKind::Http(httparse::Error::Token), format!("Not valid request.")))
    };

    let public_key_time: i64 = match public_key.parse() {
      Ok(k) => k,
      Err(_) => return Some(Error::new(ErrorKind::Http(httparse::Error::Token), format!("Not valid request. Public key is not integer: {:?}", public_key)))
    };

    if self.validate_time(public_key_time, self.auth.get_keep_alive()) == false {
      return Some(Error::new(ErrorKind::Http(httparse::Error::Token), format!("Not valid request. Expired: {:?}", public_key)))
    }

    if self.validate_token(token.as_str(), public_key.as_str()) == false {
      return Some(Error::new(ErrorKind::Http(httparse::Error::Token), format!("Not valid request. Token not valid: {}", token)))
    }

    None
  }

  fn get_token_and_public_key(&self, token_name: &str, public_key_name: &str) -> Option<(String, String)> {
    let pairs = self.url.query_pairs();

    let mut token: Option<String> = None;
    let mut public_key: Option<String> = None;

    for (key, value) in pairs {
      if key == token_name {
        token = Some(value.to_string());
      }

      if key == public_key_name {
        public_key = Some(value.to_string());
      }
    }

    match token.is_none() || public_key.is_none() {
      true => {
        error!(
          "Not found token ([name={},value={:?}]) or public key ([name={},value={:?}]) in query {:?}",
          token_name,
          token,
          public_key_name,
          public_key,
          self.url.query()
          );
        None
      },
      false => Some((token.unwrap_or("".to_string()), public_key.unwrap_or("".to_string())))
    }
  }

  fn validate_token(&self, token: &str, public_key: &str) -> bool {
    let mut auth = Hmac::new(sha1::Sha1::new(), self.auth.get_private_key().as_bytes());
    auth.input(public_key.as_bytes());

    if token != auth.result().code().to_hex() {
      error!("Token not valid. Got [{}], should [{}]", token, auth.result().code().to_hex());
    }

    token == auth.result().code().to_hex()
  }

  fn validate_time(&self, nonce: i64, keep_alive: Option<i64>) -> bool {

    let max_different_time = keep_alive.unwrap_or(120);

    if 0 == max_different_time {
      return true;
    }

    Utc::now().timestamp() - nonce < max_different_time
  }
}

#[cfg(test)]
mod test {
  use utils::HttpData;
  use chrono::{ Utc, DateTime };
  use crypto::{ hmac, sha1 };
  use crypto::mac::Mac;
  use hex::ToHex;
  use settings::auth::Authorization;

  fn get_auth_default() -> Authorization {
    Authorization {
      private_key: "usocksecret".to_string(),
      keep_alive: None,
      token_name: None,
      time_name: None
    }
  }

  #[test]
  fn test_get_group() {
    let authorization_settings: Authorization = get_auth_default();
    let data: HttpData = HttpData::new("/hello/world?jytirr", authorization_settings.clone()).unwrap();
    assert_eq!(data.get_group(), "hello/world".to_string());
    let data: HttpData = HttpData::new("/hello/world/", authorization_settings.clone()).unwrap();
    assert_eq!(data.get_group(), "hello/world".to_string());
    let data: HttpData = HttpData::new("/heLlo/worlD/", authorization_settings).unwrap();
    assert_eq!(data.get_group(), "hello/world".to_string());
  }

  #[test]
  fn test_validate_time() {
    let data: HttpData = HttpData::new("/hello/world?nonce=1504970846", get_auth_default()).unwrap();
    assert_eq!(true, data.validate_time(1504970846, Some(0)));
    assert_eq!(false, data.validate_time(1504970846, Some(120)));
    let time: DateTime<Utc> = Utc::now();
    assert_eq!(true, data.validate_time(time.timestamp(), Some(120)));
    assert_eq!(true, data.validate_time(time.timestamp() - 119i64, Some(120)));
    assert_eq!(false, data.validate_time(time.timestamp() - 120i64, Some(120)));
    assert_eq!(true, data.validate_time(time.timestamp() - 119i64, None));
  }

  #[test]
  fn test_validate_token() {
    let data: HttpData = HttpData::new("/hello/world?nonce=1504970846", get_auth_default()).unwrap();
    assert_eq!(true, data.validate_token("c3c3358c4fe308b198ee875597b16606f1c728aa", "1504970846"));
  }

  #[test]
  fn test_get_token_and_public_key() {
    let data: HttpData = HttpData::new("/hello/world?nonce=1504970846&my_token=token_value", get_auth_default()).unwrap();
    assert_eq!(Some(("token_value".to_string(), "1504970846".to_string())), data.get_token_and_public_key("my_token", "nonce"));
    assert_eq!(None, data.get_token_and_public_key("my_token", "nonc"));
    assert_eq!(None, data.get_token_and_public_key("my_toke", "nonce"));
  }

  #[test]
  fn test_validate() {
    let time = format!("{}", Utc::now().timestamp());
    let mut auth: hmac::Hmac<sha1::Sha1> = hmac::Hmac::new(sha1::Sha1::new(), "usocksecret".as_bytes());
    auth.input(time.as_bytes());

    let data: HttpData = HttpData::new(
      format!("/hello/world?nonce={}&token={}", time.as_str(), auth.result().code().to_hex()).as_str(),
      get_auth_default()
      ).unwrap();

    assert!(data.validate().is_none());
  }
}

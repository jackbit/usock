use url::Url;
use mongodb::{ Client, ThreadedClient };
use mongodb::db::ThreadedDatabase;

pub mod logger;

pub fn get_client(uri: &str, db: &str) -> Client {

  let mut uri_temp = uri.clone().to_string();
  let beta_offset = uri_temp.find(',').unwrap_or(uri_temp.len());
  let t: String = uri_temp.drain(..beta_offset).collect();

  let parsed = Url::parse(t.as_str()).unwrap();

  let client = Client::with_uri(uri)
  .expect("Failed to initialize client.");

  if let Some(pwd) = parsed.password() {
    debug!("user is [{}] and pwd is [{}]", parsed.username(), pwd);
    if let Err(e) = client.db(db).auth(parsed.username(), pwd) {
      error!("Error in auth to mongodb: {}", e);
    }
  }

  client
}

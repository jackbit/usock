#[derive(Debug, Deserialize, Clone)]
pub struct Authorization {
  pub private_key: String,
  pub keep_alive: Option<i64>,
  pub token_name: Option<String>,
  pub time_name: Option<String>
}

impl Authorization {

  pub fn get_private_key(&self) -> String {
    self.private_key.clone()
  }

  pub fn get_keep_alive(&self) -> Option<i64> {
    self.keep_alive.clone()
  }

  pub fn get_token_name(&self) -> Option<String> {
    self.token_name.clone()
  }

  pub fn get_time_name(&self) -> Option<String> {
    self.time_name.clone()
  }
}

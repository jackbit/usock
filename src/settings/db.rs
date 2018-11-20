#[derive(Debug, Deserialize, Clone)]
pub struct MongoSettings {
  db: String,
  table: String,
  uri: String
}

impl MongoSettings {

  pub fn get_db_name(&self) -> String {
    self.db.clone()
  }

  pub fn get_table_name(&self) -> String {
    self.table.clone()
  }

  pub fn get_uri(&self) -> String {
    self.uri.clone()
  }
}

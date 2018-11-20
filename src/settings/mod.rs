use config::{Config, File};
use walkdir::{DirEntry, WalkDir};

pub mod ws;
pub mod auth;
pub mod db;
pub mod rd;

#[derive(Debug, Deserialize)]
pub struct Settings {
  ws: ws::WsServer,
  auth: auth::Authorization,
  mongo: db::MongoSettings,
  rd: rd::RdConfig
}

impl Settings {
  pub fn new(run_mode: &str, path: &str) -> Self {
    let mut s = Config::new();
    for entry in WalkDir::new(&format!("{}/{}", path, run_mode))
    .max_depth(2)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok()) {
      if is_hidden(&entry) {
        continue;
      }
      if !entry.file_type().is_dir() {
        s.merge(File::with_name(&format!("{}", entry.path().display())).required(false)).unwrap();
      }
    }

    match s.try_into() {
      Ok(settings) => settings,
      Err(e) => panic!("Not found config files in path {}: {} {}", path, run_mode, e)
    }
  }

  pub fn get_ws(&self) -> &ws::WsServer {
    &self.ws
  }

  pub fn get_auth(&self) -> &auth::Authorization {
    &self.auth
  }
  pub fn get_db_mongo(&self) -> db::MongoSettings {
    self.mongo.clone()
  }

  pub fn get_rd(&self) -> rd::RdConfig {
    self.rd.clone()
  }
}

fn is_hidden(entry: &DirEntry) -> bool {
  entry.file_name()
  .to_str()
  .map(|s| s.starts_with("."))
  .unwrap_or(false)
}

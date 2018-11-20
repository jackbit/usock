#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate config;
extern crate walkdir;
extern crate ws as WebSocket;
extern crate httparse;
extern crate url;
extern crate openssl;
extern crate crypto;
extern crate chrono;
extern crate hex;
extern crate bson;
extern crate mongodb;
extern crate sidekiq;
extern crate r2d2_redis;
extern crate redis;

mod settings;
mod event;
mod ws;
mod utils;
mod db;
mod notifier;

use std::thread;
use std::sync::mpsc::channel;
use event::Event;
use ::db::get_client;
use std::env;
use notifier::{Notifier};

pub fn run(run_mode: &str, path: &str) {
  info!("Application is run");

  let config = settings::Settings::new(run_mode, path);
  let ws_connect_string: String = settings::ws::get_connect_string(config.get_ws());
  let (tx, rx) = channel::<Event>();
  let (tx_logging, rx_logging) = channel::<Event>();
  let config_db = config.get_db_mongo();
  let config_rd = config.get_rd();
  env::set_var("REDIS_NAMESPACE", config_rd.get_ns().as_str());
  env::set_var("REDIS_URL_ENV", config_rd.get_uri().as_str());
  let notifier = Notifier::new();
  notifier.connect();

  if let Err(e) = thread::Builder::new()
  .name("MongoMessageLogger".into()).spawn(
    move || {
      db::logger::logging_messages(
        rx_logging,
        get_client(config_db.get_uri().as_str(), config_db.get_db_name().as_str()),
        config_db.get_db_name(),
        config_db.get_table_name(),
        notifier
        )
    }
    ) {
    panic!("Cannot create thread to MongoMessageLogger: {}", e)
  }

  if let Err(e) = thread::Builder::new()
  .name("Multicaster".into()).spawn(
    move || {
      ws::multicast::multicast(
        rx,
        tx_logging
        )
    }
    ) {
    panic!("Cannot create thread to multicaster: {}", e)
  }

  ws::run_server(
    ws_connect_string.as_str(),
    config.get_ws().get_max_connections(),
    tx,
    config.get_ws().get_ssl(),
    config.get_auth().clone()
    )
}

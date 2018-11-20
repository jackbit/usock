extern crate log;
extern crate usock;
extern crate log4rs;
extern crate log_panics;
extern crate getopts;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::json::JsonEncoder;
use log4rs::config::{Appender, Config, Root};
use std::env;

fn main() {

  let run_mode = env::var("WS_RUN_MODE").unwrap_or("development".to_string());
  let args: Vec<String> = env::args().collect();

  let mut opts = getopts::Options::new();
  opts.optopt("p", "path", "set config path", "PATH");
  opts.optopt("l", "logging", "Flag to enable / disable file logging", "BOOL");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!("cannot parse args: {}",f.to_string())
  };

  let path = matches.opt_str("p").unwrap_or("config/".to_string());
  let logging = matches.opt_str("l").unwrap_or("true".to_string()).as_str().to_lowercase();
  println!("{:?}", matches.opt_str("l").unwrap().to_string().to_lowercase());

  if !["false", "0"].contains(&logging.as_str()) {
    let log_path: &str = &logging;

    let requests = FileAppender::builder()
    .encoder(Box::new(JsonEncoder::new()))
    .build(log_path)
    .unwrap();

    log_panics::init();

    let config = Config::builder()
    .appender(Appender::builder().build("requests", Box::new(requests)))
    .build(Root::builder().appender("requests").build(match run_mode.as_str() {
      "production" => LogLevelFilter::Warn,
      "staging" => LogLevelFilter::Warn,
      _ => LogLevelFilter::Debug
    }))
    .unwrap();

    log4rs::init_config(config).unwrap();
  }

  usock::run(&run_mode, path.as_str());
}

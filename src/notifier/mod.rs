use std::env;
use sidekiq::{Client, ClientOpts, JobOpts, Job};
use bson::ordered::{ OrderedDocument };
use r2d2_redis::{redis, RedisConnectionManager};
use r2d2_redis::r2d2::{Pool};
use std::error::Error;

pub struct Notifier {
  client: Client
}

impl Notifier {
  pub fn new() -> Self {
    let client_opts = ClientOpts {
      namespace: None,
      ..Default::default()
    };

    let redis_url = env::var("REDIS_URL_ENV").unwrap().to_string();
    let url = redis::parse_redis_url(&redis_url).unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    let redis_pool = Pool::new(manager).map_err(|err| err.description().to_string());

    Notifier {
      client: Client::new(redis_pool.unwrap(), client_opts)
    }
  }

  pub fn connect(&self) {
    if let Err(err) = self.client.redis_pool.get() {
      panic!("Cannot create thread to RedisPool: {}", err)
    }
  }

  pub fn publish(&self, value: &OrderedDocument) {
    let job_args = json!(value);
    let job_options = JobOpts {
      queue: "notification".to_string(),
      ..Default::default()
    };

    let job = Job::new("ChatNotifier".to_string(), vec![job_args], job_options);

    if let Err(err) = self.client.push(job) {
      error!("SidekiqClient push failed: {}", err);
    }
  }
}

use WebSocket::Sender;
use chrono::prelude::*;
use bson::Bson;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventMessage {
  pub created_at: Bson,
  pub channel: String,
  pub message: String,
  pub ip: String
}

impl EventMessage {
  pub fn new(channel: String, message: String, ip: String) -> Self {
    EventMessage {
      created_at: Bson::from(Utc::now()),
      channel: channel,
      message: message,
      ip: ip
    }
  }
}

pub struct MultiCastMessage {
  pub id: String,
  pub message: EventMessage
}

impl MultiCastMessage {
  pub fn new(channel: String, id: String, message: String, ip: String) -> Self {
    MultiCastMessage {
      message: EventMessage::new(channel, message, ip),
      id: id
    }
  }
}

pub enum Event {
  Subscribe((String, Sender, String)),
  UnSubscribe((String, String)),
  Multicast(MultiCastMessage),
  Logging(EventMessage)
}

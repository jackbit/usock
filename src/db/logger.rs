use std::sync::mpsc::Receiver;
use event::Event;
use mongodb::{ Client, ThreadedClient };
use mongodb::db::ThreadedDatabase;
use bson::{ to_bson, Bson };
use notifier::{ Notifier };

pub fn logging_messages(rx: Receiver<Event>, db: Client, db_name: String, table_name: String, notifier: Notifier) {
  loop {
    match rx.recv() {
      Ok(Event::Logging(m)) => {
        if let Ok(Bson::Document(document)) = to_bson(&m) {
          notifier.publish(&document);
          if let Err(e) = db.db(db_name.as_str()).collection(table_name.as_str()).insert_one(document, None) {
            error!("Cannot insert into mongo: {}", e);
          }
        }
      },
      _ => ()
    }
  }
}

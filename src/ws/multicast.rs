use std::collections::HashMap;
use WebSocket::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender as ThreadSender;
use event::Event;

pub fn multicast(rx: Receiver<Event>, tx: ThreadSender<Event>) {

  let mut rooms: HashMap<String, HashMap<String, Sender>> = HashMap::new();

  loop {
    match rx.recv() {
      Ok(Event::Subscribe((id, out, group))) => {
        if let None = rooms.get_mut(group.as_str()) {
          rooms.insert(group.clone(), HashMap::new());
        }
        if let Some(room) = rooms.get_mut(group.as_str()) {
          room.insert(id, out);
        }
      },
      Ok(Event::UnSubscribe((id, group))) => {
        if let Some(room) = rooms.get_mut(group.as_str()) {
          room.remove(&id);
        }
      },
      Ok(Event::Multicast(message)) => {

        if let Err(e) = tx.send(Event::Logging(message.message.clone())) {
          error!("{}", e);
        }

        let channel = message.message.channel.as_str();

        match rooms.get(channel) {
          Some(room) => {
            for (user, out) in room {
              if user != &message.id {
                if let Err(e) = out.send(message.message.message.as_str()) {
                  error!("{}", e);
                }
              }
            }
          },
          _ => {
            error!("Undefined room [{}]", message.message.channel.as_str());
          }
        }
      },
      Err(_) => panic!("MultiCast die"),
      _ => {

      }
    }
  }
}

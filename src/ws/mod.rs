use std::sync::mpsc::Sender as ThreadSender;
use WebSocket::{Builder, Settings, Sender};
use ::event::Event;
use std::rc::Rc;
use ::settings::ws::Ssl;
use std::io::Read;
use std::fs::File;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod };
use openssl::pkey::PKey;
use openssl::x509::{X509, X509Ref};
use std;
use settings::auth::Authorization;

mod server;
pub mod multicast;

pub fn run_server(connect_str: &str, max_connections: usize, tx: ThreadSender<Event>, ssl_config: Option<Ssl>, auth: Authorization) {
  let ssl: Option<Rc<SslAcceptor>> = match ssl_config {
    Some(ssl) => {
      let cert = {
        let data = read_file(ssl.cert.as_str()).unwrap();
        X509::from_pem(data.as_ref()).unwrap()
      };
      let pkey = {
        let data = read_file(ssl.key.as_str()).unwrap();
        PKey::private_key_from_pem(data.as_ref()).unwrap()
      };
      let acceptor = Rc::new(SslAcceptorBuilder::mozilla_intermediate(
        SslMethod::tls(),
        &pkey,
        &cert,
        std::iter::empty::<X509Ref>(),
        ).unwrap().build());
      Some(acceptor)
    },
    _ => None
  };
  Builder::new().with_settings(Settings {
    max_connections: max_connections,
    panic_on_internal: false,
    encrypt_server: match ssl {
      Some(_) => true,
      _ => false
    },
    ..Settings::default()
  }).build(|out: Sender| {
    server::Server::new(out,tx.clone(), ssl.clone(), auth.clone())
  }).unwrap().listen(connect_str).unwrap();
}

fn read_file(name: &str) -> std::io::Result<Vec<u8>> {
  let mut file = try!(File::open(name));
  let mut buf = Vec::new();
  try!(file.read_to_end(&mut buf));
  Ok(buf)
}

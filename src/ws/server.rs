use WebSocket::{ CloseCode, Sender, Handler, Message, Result, Handshake, Request, Response, ErrorKind, Error };
use std::sync::mpsc::Sender as ThreadSender;
use ::event::{ Event, MultiCastMessage };
use utils::HttpData;
use std::rc::Rc;
use openssl::ssl::SslAcceptor;
use openssl::ssl::SslStream;
use WebSocket::util::TcpStream;
use settings::auth::Authorization;

pub struct Server {
  out: Sender,
  extern_out: ThreadSender<Event>,
  id: String,
  group: String,
  ssl: Option<Rc<SslAcceptor>>,
  auth: Authorization,
  ip: String
}

impl Server {
  pub fn new(out: Sender, extern_out:ThreadSender<Event>, ssl: Option<Rc<SslAcceptor>>, auth: Authorization) -> Self {
    Server {
      out: out,
      extern_out: extern_out,
      id: "".to_string(),
      group: "".to_string(),
      ssl: ssl,
      auth: auth,
      ip: "127.0.0.1".to_string()
    }
  }
}

impl Handler for Server {

  fn on_open(&mut self, shake: Handshake) -> Result<()> {

    if let Ok(Some(ip_addr)) = shake.remote_addr() {
      self.ip = ip_addr.to_string()
    }

    if let Err(e) = self.extern_out.send(Event::Subscribe((self.id.clone(), self.out.clone(), self.group.clone()))) {
      error!("{}", e)
    }

    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    if let Err(e) = self.extern_out
    .send(Event::Multicast(MultiCastMessage::new(self.group.clone(), self.id.clone(), format!("{}", msg), self.ip.clone()))) {
      error!("{}", e)
    }

    Ok(())
  }

  fn on_close(&mut self, _: CloseCode, _: &str) {
    if let Err(e) = self.extern_out.send(Event::UnSubscribe((self.id.clone(), self.group.clone()))) {
      error!("{}", e)
    }
  }

  fn on_request(&mut self, req: &Request) -> Result<Response> {

    let uri: HttpData = HttpData::new(
      req.resource(),
      self.auth.clone()
      )?;

    if let Some(e) = uri.validate() {
      return Err(e)
    }

    self.group = uri.get_group();

    if let Some(Ok(id)) = req.header("Sec-WebSocket-Key").map(|id| String::from_utf8(id.clone())) {
      self.id = id;
    }

    Response::from_request(req)
  }

  fn upgrade_ssl_server(&mut self, sock: TcpStream) -> Result<SslStream<TcpStream>> {
    match self.ssl.as_ref() {
      Some(ssl) => ssl.accept(sock).map_err(From::from),
      _ => Err(Error::new(ErrorKind::Internal, "Ssl not found"))
    }
  }
}

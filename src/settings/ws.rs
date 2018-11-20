#[derive(Debug, Deserialize)]
pub struct WsServer {
  ssl: Option<Ssl>,
  host: String,
  port: u16,
  max_connections: usize
}

impl WsServer {
  pub fn get_ssl(&self) -> Option<Ssl> {
    self.ssl.clone()
  }

  pub fn get_host(&self) -> String {
    self.host.clone()
  }

  pub fn get_port(&self) -> u16 {
    self.port.clone()
  }

  pub fn get_max_connections(&self) -> usize {
    self.max_connections.clone()
  }
}

pub fn get_connect_string(settings: &WsServer) -> String {
  format!("{}:{}", settings.get_host(), settings.get_port())
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ssl {
  pub key: String,
  pub cert: String
}

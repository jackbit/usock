#[derive(Debug, Deserialize, Clone)]
pub struct RdConfig {
	pub uri: String,
	pub ns: String,
}

impl RdConfig {

	pub fn get_uri(&self) -> String {
		self.uri.clone()
	}

	pub fn get_ns(&self) -> String {
		self.ns.clone()
	}
}

use reqwest::Client;

#[derive(Debug, Clone)]
pub struct HttpClientModel {
	pub client: Client,
}

impl HttpClientModel {
	pub fn create_http_client() -> Self {
		let client = reqwest::Client::new();
		Self { client }
	}
}

use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Link {
	id: String,
	text: String,
	url: String,
}

impl Link {
	pub fn new(text: &str, url: &str) -> Self {
		Self {
			id: "link".to_string(),
			text: text.to_string(),
			url: url.to_string(),
		}
	}
}

impl Component for Link {}

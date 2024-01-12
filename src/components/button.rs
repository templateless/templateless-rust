use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Button {
	id: String,
	text: String,
	url: String,
}

impl Button {
	pub fn new(text: &str, url: &str) -> Self {
		Self {
			id: "button".to_string(),
			text: text.to_string(),
			url: url.to_string(),
		}
	}
}

impl Component for Button {}

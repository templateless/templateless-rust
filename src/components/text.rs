use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Text {
	id: String,
	text: String,
}

impl Text {
	pub fn new(text: &str) -> Self {
		Self { id: "text".to_string(), text: text.to_string() }
	}
}

impl Component for Text {}

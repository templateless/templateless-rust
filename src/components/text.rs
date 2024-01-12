use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Text {
	id: String,
	text: String,
}

impl Text {
	pub fn new(input: &str) -> Self {
		Self { id: "text".to_string(), text: input.to_string() }
	}
}

impl Component for Text {}

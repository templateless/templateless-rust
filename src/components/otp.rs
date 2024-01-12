use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Otp {
	id: String,
	text: String,
}

impl Otp {
	pub fn new(text: &str) -> Self {
		Self { id: "otp".to_string(), text: text.to_string() }
	}
}

impl Component for Otp {}

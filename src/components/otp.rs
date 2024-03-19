use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Otp {
	id: ComponentId,
	text: String,
}

impl Otp {
	pub fn new(text: &str) -> Self {
		Self { id: ComponentId::Otp, text: text.to_string() }
	}
}

impl Component for Otp {}

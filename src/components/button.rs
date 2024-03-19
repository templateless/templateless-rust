use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Button {
	id: ComponentId,
	text: String,
	url: String,
}

impl Button {
	pub fn new(text: &str, url: &str) -> Self {
		Self {
			id: ComponentId::Button,
			text: text.to_string(),
			url: url.to_string(),
		}
	}
}

impl Component for Button {}

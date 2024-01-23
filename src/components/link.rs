use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct Link {
	id: ComponentId,
	text: String,
	url: String,
}

impl Link {
	pub fn new(text: &str, url: &str) -> Self {
		Self {
			id: ComponentId::Link,
			text: text.to_string(),
			url: url.to_string(),
		}
	}
}

impl Component for Link {}

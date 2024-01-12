use serde::Serialize;

use crate::components::{Component, Text};

#[derive(Clone, Serialize)]
pub struct Template {
	version: u8,
	theme: String,
	header: Vec<Box<dyn Component>>,
	body: Vec<Vec<Box<dyn Component>>>,
	footer: Vec<Box<dyn Component>>,
}

impl Template {
	pub fn builder() -> Self {
		Self {
			version: 1,
			theme: "simple".to_string(),
			header: vec![],
			body: vec![],
			footer: vec![],
		}
	}

	pub fn text(mut self, input: &str) -> Self {
		let text = Box::new(Text::new(input));
		self.body.push(vec![text]);
		self
	}
}

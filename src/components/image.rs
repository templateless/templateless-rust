use serde::Serialize;

use crate::{
	components::{Component, ComponentId},
	Result,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
	id: ComponentId,
	src: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	width: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	height: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	alt: Option<String>,
}

impl Image {
	pub fn new(src: &str) -> Self {
		Self {
			id: ComponentId::Image,
			src: src.to_string(),
			url: None,
			width: None,
			height: None,
			alt: None,
		}
	}

	pub fn url(mut self, url: &str) -> Self {
		self.url = Some(url.to_string());
		self
	}

	pub fn width(mut self, width: usize) -> Self {
		self.width = Some(width);
		self
	}

	pub fn height(mut self, height: usize) -> Self {
		self.height = Some(height);
		self
	}

	pub fn alt(mut self, alt: &str) -> Self {
		self.alt = Some(alt.to_string());
		self
	}

	pub fn build(&self) -> Result<Self> {
		Ok(self.clone())
	}
}

impl Component for Image {}

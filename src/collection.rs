use serde::Serialize;

use crate::{
	components::{Button, Component, Image, Link, List, Otp, Socials, Text},
	ListStyle, SocialItem,
};

#[derive(Clone, Serialize)]
pub struct Collection {
	pub components: Vec<Box<dyn Component>>,
}

impl Collection {
	pub fn builder() -> Self {
		Self { components: vec![] }
	}

	pub fn button(self, text: &str, url: &str) -> Self {
		self.push(Box::new(Button::new(text, url)))
	}

	pub fn image(
		self,
		src: &str,
		url: Option<String>,
		width: Option<usize>,
		height: Option<usize>,
		alt: Option<String>,
	) -> Self {
		self.push(Box::new(Image::new(
			src.to_string(),
			alt.unwrap_or_default(),
			width.unwrap_or_default(),
			height.unwrap_or_default(),
			url.unwrap_or_default(),
		)))
	}

	pub fn link(self, text: &str, url: &str) -> Self {
		self.push(Box::new(Link::new(text, url)))
	}

	pub fn list(
		self,
		style: ListStyle,
		items: Vec<Box<dyn Component>>,
	) -> Self {
		self.push(Box::new(List::new(style, items)))
	}

	pub fn otp(self, text: &str) -> Self {
		self.push(Box::new(Otp::new(text)))
	}

	pub fn socials(self, data: Vec<SocialItem>) -> Self {
		self.push(Box::new(Socials::new(data)))
	}

	pub fn text(self, text: &str) -> Self {
		self.push(Box::new(Text::new(text)))
	}

	pub fn build(self) -> Self {
		self
	}

	fn push(mut self, component: Box<dyn Component>) -> Self {
		self.components.push(component);
		self
	}
}

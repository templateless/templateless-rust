use serde::Serialize;

use crate::{
	components::{
		Button, Component, Image, Link, Otp, Socials, Text, ViewInBrowser,
	},
	Result, SocialItem,
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
		self.push(Box::new(Image::new(src, url, width, height, alt)))
	}

	pub fn link(self, text: &str, url: &str) -> Self {
		self.push(Box::new(Link::new(text, url)))
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

	pub fn view_in_browser(self, text: &str) -> Self {
		self.push(Box::new(ViewInBrowser::new(text)))
	}

	pub fn build(&self) -> Result<Self> {
		Ok(self.clone())
	}

	fn push(mut self, component: Box<dyn Component>) -> Self {
		self.components.push(component);
		self
	}
}

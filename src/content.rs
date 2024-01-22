use serde::Serialize;

use crate::{
	components::{Button, Component, Image, Link, List, Otp, Socials, Text},
	Footer, Header, ListStyle, SocialItem,
};

#[derive(Clone, Serialize)]
pub enum Theme {
	#[serde(rename = "unstyled")]
	Unstyled,
	#[serde(rename = "simple")]
	Simple,
}

#[derive(Clone, Serialize)]
pub struct Content {
	version: u8,
	theme: Theme,
	header: Vec<Box<dyn Component>>,
	body: Vec<Vec<Box<dyn Component>>>,
	footer: Vec<Box<dyn Component>>,
}

impl Content {
	pub fn builder() -> Self {
		Self {
			version: 0,
			theme: Theme::Unstyled,
			header: vec![],
			body: vec![vec![]],
			footer: vec![],
		}
	}

	pub fn theme(mut self, theme: Theme) -> Self {
		self.theme = theme;
		self
	}

	pub fn header(mut self, header: Header) -> Self {
		self.header = header.components;
		self
	}

	pub fn footer(mut self, footer: Footer) -> Self {
		self.footer = footer.components;
		self
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
		self.body[0].push(component);
		self
	}
}

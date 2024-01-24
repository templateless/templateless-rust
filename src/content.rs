use serde::Serialize;

use crate::{
	components::{
		Button, Component, Image, Link, Otp, Socials, Text, ViewInBrowser,
	},
	Footer, Header, Result, SocialItem,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Theme {
	Unstyled,
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
		self.body[0].push(component);
		self
	}
}

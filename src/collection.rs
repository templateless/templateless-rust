use serde::Serialize;

use crate::{
	components::{
		Button, Component, Image, Link, Otp, QrCode, Signature, SocialItem,
		Socials, StoreBadgeItem, StoreBadges, Text, ViewInBrowser,
	},
	Result,
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

	pub fn image(self, src: &str) -> Self {
		self.push(Box::new(Image::new(src)))
	}

	pub fn link(self, text: &str, url: &str) -> Self {
		self.push(Box::new(Link::new(text, url)))
	}

	pub fn otp(self, text: &str) -> Self {
		self.push(Box::new(Otp::new(text)))
	}

	pub fn socials(self, data: &[SocialItem]) -> Self {
		self.push(Box::new(Socials::new(data.to_vec())))
	}

	pub fn text(self, text: &str) -> Self {
		self.push(Box::new(Text::new(text)))
	}

	pub fn view_in_browser(self) -> Self {
		self.push(Box::new(ViewInBrowser::new(None)))
	}

	pub fn qr_code(self, url: &str) -> Self {
		self.push(Box::new(QrCode::url(url)))
	}

	pub fn store_badges(self, data: &[StoreBadgeItem]) -> Self {
		self.push(Box::new(StoreBadges::new(data.to_vec())))
	}

	pub fn signature(self, text: &str) -> Self {
		self.push(Box::new(Signature::new(text, None)))
	}

	pub fn component<T>(self, c: T) -> Self
	where
		T: Component + 'static,
	{
		self.push(Box::new(c))
	}

	pub fn build(&self) -> Result<Self> {
		Ok(self.clone())
	}

	fn push(mut self, component: Box<dyn Component>) -> Self {
		self.components.push(component);
		self
	}
}

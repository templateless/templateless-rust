use erased_serde::serialize_trait_object;
use serde::Serialize;

pub use button::Button;
pub use image::Image;
pub use link::Link;
pub use otp::Otp;
pub use socials::{Item as SocialItem, Service, Socials};
pub use text::Text;
pub use view_in_browser::ViewInBrowser;

mod button;
mod image;
mod link;
mod otp;
mod socials;
mod text;
mod view_in_browser;

#[derive(
	Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentId {
	Button,
	Image,
	Link,
	Otp,
	PoweredBy,
	Socials,
	Text,
	ViewInBrowser,
}

pub trait Component: ComponentClone + erased_serde::Serialize + Send {}

pub trait ComponentClone {
	fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
	T: 'static + Component + Clone,
{
	fn clone_box(&self) -> Box<dyn Component> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Component> {
	fn clone(&self) -> Box<dyn Component> {
		self.clone_box()
	}
}

serialize_trait_object!(Component);

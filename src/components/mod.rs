use erased_serde::serialize_trait_object;
use serde::Serialize;

pub use button::Button;
pub use image::Image;
pub use link::Link;
pub use otp::Otp;
pub use qr_code::QrCode;
pub use signature::{Font as SignatureFont, Signature};
pub use socials::{Item as SocialItem, Service, Socials};
pub use store_badges::{Item as StoreBadgeItem, StoreBadge, StoreBadges};
pub use text::Text;
pub use view_in_browser::ViewInBrowser;

mod button;
mod image;
mod link;
mod otp;
mod qr_code;
mod signature;
mod socials;
mod store_badges;
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
	QrCode,
	StoreBadges,
	Signature,
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

use erased_serde::serialize_trait_object;

pub use button::Button;
pub use image::Image;
pub use link::Link;
pub use list::{List, Style as ListStyle};
pub use otp::Otp;
pub use socials::{Item as SocialItem, Service, Socials};
pub use text::Text;

mod button;
mod image;
mod link;
mod list;
mod otp;
mod socials;
mod text;

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

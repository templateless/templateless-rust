use erased_serde::serialize_trait_object;

pub use text::Text;

mod text;

pub trait Component: ComponentClone + erased_serde::Serialize {}

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

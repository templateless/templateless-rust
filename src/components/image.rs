use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct Image {
	id: ComponentId,
	src: String,
	url: Option<String>,
	width: Option<usize>,
	height: Option<usize>,
	alt: Option<String>,
}

impl Image {
	pub fn new(
		src: &str,
		url: Option<String>,
		width: Option<usize>,
		height: Option<usize>,
		alt: Option<String>,
	) -> Self {
		Self {
			id: ComponentId::Image,
			src: src.to_string(),
			url,
			width,
			height,
			alt,
		}
	}
}

impl Component for Image {}

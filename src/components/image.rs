use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct Image {
	id: ComponentId,
	src: String,
	alt: Option<String>,
	width: Option<usize>,
	height: Option<usize>,
	url: Option<String>,
}

impl Image {
	pub fn new(
		src: &str,
		alt: Option<String>,
		width: Option<usize>,
		height: Option<usize>,
		url: Option<String>,
	) -> Self {
		Self {
			id: ComponentId::Image,
			src: src.to_string(),
			alt,
			width,
			height,
			url,
		}
	}
}

impl Component for Image {}

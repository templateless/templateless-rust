use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct Image {
	id: ComponentId,
	src: String,
	alt: String,
	width: usize,
	height: usize,
	url: String,
}

impl Image {
	pub fn new(
		src: String,
		alt: String,
		width: usize,
		height: usize,
		url: String,
	) -> Self {
		Self {
			id: ComponentId::Image,
			src: src.to_string(),
			alt: alt.to_string(),
			width,
			height,
			url: url.to_string(),
		}
	}
}

impl Component for Image {}

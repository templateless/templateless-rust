use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Image {
	id: String,
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
			id: "image".to_string(),
			src: src.to_string(),
			alt: alt.to_string(),
			width,
			height,
			url: url.to_string(),
		}
	}
}

impl Component for Image {}

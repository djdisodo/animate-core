use std::path::Path;
use serde_derive::*;
use image::RgbaImage;
use crate::element::{Element, FrameLength};
use crate::{BaseSigned, BaseUnsigned, Context, Graphics};
use std::time::{Instant, Duration};
use cgmath::Vector2;
use std::lazy::OnceCell;
use graphics::ImageSize;
use derivative::Derivative;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Image {
	path: String,
	#[derivative(Debug="ignore")]
	#[serde(skip)]
	inner: OnceCell<<Graphics as graphics::Graphics>::Texture>
}

impl Clone for Image {
	fn clone(&self) -> Self {
		Self {
			path: self.path.clone(),
			inner: OnceCell::default()
		}
	}
}

impl Image {
	pub fn new(path: String) -> Self {
		Self {
			path,
			inner: OnceCell::new()
		}
	}

	fn init(&self) {
		self.inner.get_or_init(|| {
			let image = image::io::Reader::open(&self.path).unwrap().decode().unwrap().into_rgba8();
			//TODO log error
			<Graphics as graphics::Graphics>::Texture::from_image(&image, &opengl_graphics::TextureSettings::new())
		});
	}
}

impl Element for Image {

	fn get_size(&self, context: &Context) -> (Vector2<BaseUnsigned>, FrameLength) {
		if let Some(image) = self.inner.get() {
			(Vector2::new(image.get_width(), image.get_height()), FrameLength::Forever)
		} else {
			self.init();
			self.get_size(context)
		}
	}

	fn draw(&self, context: &Context, graphics: &mut Graphics) -> FrameLength {
		if let Some(image) = self.inner.get() {
			graphics::image(image, context.transform, graphics);
			FrameLength::Forever
		} else {
			self.init();
			self.draw(context, graphics)
		}
	}
}
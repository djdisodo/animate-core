use serde_derive::*;
use crate::element::{Element, FrameLength};
use crate::{BaseUnsigned, Context};
use cgmath::Vector2;
use std::lazy::OnceCell;
use derivative::Derivative;
use sfml::graphics::{RenderTarget, Texture, Sprite};
use sfml::system::SfBox;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Image {
	path: String,
	#[derivative(Debug="ignore")]
	#[serde(skip)]
	inner: OnceCell<SfBox<Texture>>
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
			Texture::from_file(&self.path).unwrap()
		});
	}
}

impl Element for Image {

	fn get_size(&self, context: &Context) -> (Vector2<BaseUnsigned>, FrameLength) {
		if let Some(image) = self.inner.get() {
			(Vector2::new(image.size().x, image.size().y), FrameLength::Forever)
		} else {
			self.init();
			self.get_size(context)
		}
	}

	fn draw(&self, context: &Context, graphics: &mut dyn RenderTarget) -> FrameLength {
		if let Some(image) = self.inner.get() {
			let sprite = Sprite::with_texture(image);
			graphics.draw_with_renderstates(&sprite, context.render_states);
			FrameLength::Forever
		} else {
			self.init();
			self.draw(context, graphics)
		}
	}
}
use crate::element::{DynElement, Element, FrameLength};
use cgmath::Vector2;
use crate::{BaseUnsigned, Context, ValueProvider};
use serde_derive::*;
use std::cmp::min;
use sfml::graphics::RenderTarget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
	#[serde(with = "serde_traitobject")]
	pub inner: DynElement,
	#[serde(with = "serde_traitobject")]
	pub position: Box<dyn ValueProvider<Vector2<BaseUnsigned>>>
}

impl Element for Position {

	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength) {
		let (position, frame_length) = self.position.get(context);
		let (inner_size, inner_frame_length) = self.inner.get_size(context);
		(position + inner_size, min(frame_length, inner_frame_length))
	}

	fn draw(&self, context: &Context, graphics: &mut dyn RenderTarget) -> FrameLength {
		let (position, frame_length) = self.position.get(context);
		let mut context = context.clone();
		context.render_states.transform.translate(position.x as _, position.y as _);
		min(self.inner.draw(&context, graphics), frame_length)
	}
}


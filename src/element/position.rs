use crate::element::{DynElement, Element, FrameLength};
use cgmath::Vector2;
use crate::{BaseUnsigned, Context, Graphics, ValueProvider};
use serde_derive::*;
use graphics::Transformed;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
	pub inner: DynElement,
	#[serde(with = "serde_traitobject")]
	pub position: Box<dyn ValueProvider<Vector2<BaseUnsigned>>>
}

impl Element for Position {

	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength) {
		let (inner_size, frame_length) = self.inner.get_size(context);
		(self.position.get(context) + inner_size, frame_length)
	}

	fn draw(&self, context: &Context, graphics: &mut Graphics) -> FrameLength {
		let mut context = context.clone();
		context.transform = context.transform.trans(self.position.x as _, self.position.y as _);
		self.inner.draw(&context, graphics)
	}
}


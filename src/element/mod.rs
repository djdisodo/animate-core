mod image;
mod position;
mod resize;
mod timeline;



pub use self::image::Image;
pub use position::Position;

pub use timeline::*;

use std::time::{Instant, Duration};
use crate::{ValueProvider, BaseSigned, BaseUnsigned, Graphics, Context};
use cgmath::Vector2;
use serde_derive::*;
use serde_traitobject::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use dyn_clone::DynClone;
use std::fmt::Debug;
use std::ops::Deref;

pub trait Element: DynClone + Debug + Serialize + Deserialize + 'static {
	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength);

	fn draw(&self, context: &Context, graphics: &mut Graphics) -> FrameLength; // next update time
}

dyn_clone::clone_trait_object!(Element);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynElement {
	#[serde(with = "serde_traitobject")]
	inner: Box<dyn Element>
}

impl DynElement {
	pub fn new(inner: impl Element) -> Self {
		Self {
			inner: Box::new(inner)
		}
	}
}

impl Deref for DynElement {
	type Target = dyn Element;

	fn deref(&self) -> &Self::Target {
		self.inner.deref()
	}
}

impl Element for DynElement {
	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength) {
		(*self).get_size(context)
	}

	fn draw(&self, context: &Context, graphics: &mut Graphics) -> FrameLength {
		(*self).draw(context, graphics)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameLength {
	Consistent,
	Forever,
	Limited(Duration)
}
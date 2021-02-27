mod image;
mod position;
mod resize;
mod timeline;



pub use self::image::Image;
pub use position::Position;
pub use timeline::*;

use std::time::Duration;
use crate::Context;
use cgmath::Vector2;
use serde_derive::*;
use serde_traitobject::{Serialize, Deserialize};
use dyn_clone::DynClone;
use std::fmt::Debug;
use std::cmp::Ordering;
use sfml::graphics::RenderTarget;

pub trait Element: DynClone + Debug + Serialize + Deserialize + 'static {
	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength);

	fn draw(&self, context: &Context, graphics: &mut dyn RenderTarget) -> FrameLength; // next update time
}

dyn_clone::clone_trait_object!(Element);

pub type DynElement = Box<dyn Element>;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FrameLength {
	Consistent,
	Forever,
	Limited(Duration),
	End
}

impl PartialOrd for FrameLength {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self == other {
			return if *self == Self::End {
				None
			} else {
				Some(Ordering::Equal)
			};
		}
		Some(match self {
			Self::Consistent => Ordering::Less,
			Self::Forever => Ordering::Greater,
			Self::Limited(duration) => match other {
				Self::Consistent => Ordering::Greater,
				Self::Forever => Ordering::Less,
				Self::Limited(other_duration) => return duration.partial_cmp(other_duration),
				Self::End => Ordering::Greater
			},
			Self::End => Ordering::Less
		})
	}
}

impl Ord for FrameLength {
	fn cmp(&self, other: &Self) -> Ordering {
		if self == other {
			return Ordering::Equal
		}
		match self {
			Self::Consistent => Ordering::Less,
			Self::Forever => Ordering::Greater,
			Self::Limited(duration) => match other {
				Self::Consistent => Ordering::Greater,
				Self::Forever => Ordering::Less,
				Self::Limited(other_duration) => duration.cmp(other_duration),
				Self::End => Ordering::Greater
			},
			Self::End => Ordering::Less
		}
	}
}
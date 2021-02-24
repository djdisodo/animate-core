use std::time::Duration;
use std::ops::Range;
use crate::element::{Element, FrameLength, DynElement};
use serde_derive::*;
use crate::{Context, Graphics};
use cgmath::Vector2;
use std::cmp::min;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
	elements: Vec<(Range<Duration>, TimelineElement)>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineElement {
	pub start: Duration,
	pub inner: DynElement
}

impl TimelineElement {
	pub fn new(element: impl Element) -> Self {
		Self {
			start: Duration::default(),
			inner: DynElement::new(element)
		}
	}
}

impl Timeline {
	pub fn append(&mut self, element: TimelineElement, to: Range<Duration>) {
		for (range, mut timeline_element) in std::mem::replace(&mut self.elements, Vec::new()) {
			let start = to.contains(&range.start);
			let end = to.contains(&range.end);
			if !start && !end {
				if range.contains(&to.start) {
					self.elements.push((range.start..to.start, timeline_element.clone()));
					timeline_element.start += to.end - range.start;
					self.elements.push((to.end..range.end, timeline_element));
				} else {
					self.elements.push((range, timeline_element));
				}
			} else if !end {
				timeline_element.start += to.end - range.start;
				self.elements.push((to.end..range.end, timeline_element));
			} else if !start {
				self.elements.push((range.start..to.start, timeline_element));
			}
		}
	}

	pub fn push(&mut self, element: TimelineElement, to: Range<Duration>) {
		//TODO
	}

	pub fn shift_and_insert(&mut self, element: TimelineElement, to: Range<Duration>) {
		//TODO
	}

	pub fn get_elements(&self) -> &Vec<(Range<Duration>, TimelineElement)> {
		&self.elements
	}

	fn get_context_and_element(&self, context: &Context) -> SeekResult {
		for (range, timeline_element) in &self.elements {
			if range.contains(&context.duration_offset) {
				let mut new_context = context.clone();
				new_context.duration_offset -= range.start - timeline_element.start;
				return SeekResult::Element {
					context: new_context,
					element: (&range, &timeline_element.inner)
				};
			}

			if range.start > context.duration_offset {
				return SeekResult::Empty {
					next_element: Some((&range, &timeline_element.inner))
				};
			}
		}
		SeekResult::Empty {
			next_element: None
		}
	}
}

enum SeekResult<'a> {
	Element {
		context: Context,
		element: (&'a Range<Duration>, &'a DynElement)
	},
	Empty {
		next_element: Option<(&'a Range<Duration>, &'a DynElement)>
	}
}

impl Element for Timeline {
	fn get_size(&self, context: &Context) -> (Vector2<u32>, FrameLength) {
		match self.get_context_and_element(context) {
			SeekResult::Element {
				context: new_context,
				element: (range, element)
			} => {
				let (size, frame_length) = element.get_size(&new_context);
				match frame_length {
					FrameLength::Forever => {
						(size, FrameLength::Limited(range.end - context.duration_offset))
					},
					FrameLength::Consistent => {
						(size, frame_length)
					},
					FrameLength::Limited(length) => {
						(size, FrameLength::Limited(min(range.end - context.duration_offset, length)))
					}
				}
			},
			SeekResult::Empty { next_element } => if let Some((range, next_element)) = next_element {
				(Vector2::new(0, 0), FrameLength::Limited(range.start - context.duration_offset))
			} else {
				(Vector2::new(0, 0), FrameLength::Forever)
			}
		}
	}

	fn draw(&self, context: &Context, graphics: &mut Graphics) -> FrameLength {
		match self.get_context_and_element(context) {
			SeekResult::Element {
				context: new_context,
				element: (range, element)
			} => {
				match element.draw(&new_context, graphics) {
					FrameLength::Forever => {
						FrameLength::Limited(range.end - context.duration_offset)
					},
					FrameLength::Consistent => FrameLength::Consistent,
					FrameLength::Limited(length) => {
						FrameLength::Limited(min(range.end - context.duration_offset, length))
					}
				}
			},
			SeekResult::Empty { next_element } => if let Some((range, next_element)) = next_element {
				FrameLength::Limited(range.start - context.duration_offset)
			} else {
				FrameLength::Forever
			}
		}
	}
}
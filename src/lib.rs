#![feature(once_cell, type_ascription)]

pub mod element;

use std::time::Duration;
use dyn_clone::DynClone;
use std::fmt::Debug;
use serde_traitobject::*;
use serde_derive::*;
use crate::element::FrameLength;
use sfml::graphics::RenderStates;

pub type BaseSigned = i32;
pub type BaseUnsigned = u32;

pub trait ValueProvider<T = BaseSigned>: DynClone + Debug + Serialize + Deserialize + 'static {
    fn get(&self, context: &Context) -> (T, FrameLength);
}
dyn_clone::clone_trait_object!(<T> ValueProvider<T>);

impl<T: Copy + DynClone + Debug + Serialize + Deserialize + 'static> ValueProvider<T> for T {
    fn get(&self, _: &Context) -> (T, FrameLength) {
        (*self, FrameLength::Forever)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuaScript {
    script: String
}

#[derive(Clone)]
pub struct Context {
    duration_from_start: Duration,
    duration_offset: Duration,
    render_states: RenderStates<'static, 'static, 'static>
}

#[cfg(test)]
fn main() {
    tests::main()
}

#[cfg(test)]
mod tests {
    use crate:: Context;
    use crate::element::{Image, Element, Position, Timeline, TimelineElement, FrameLength};
    use cgmath::Vector2;
    use std::thread::sleep;
    use std::time::{Duration, Instant};
    use sfml::graphics::{RenderTexture, RenderWindow, RenderStates, RenderTarget, Sprite, Color};
    use sfml::window::{Style, ContextSettings, Event};

    pub fn main() {
        let mut render_texture = RenderTexture::new(1280, 720, false).unwrap();
        let mut render_window = RenderWindow::new((1280, 720), "hello", Style::CLOSE, &ContextSettings::default());

        render_window.set_framerate_limit(60);
        render_window.set_active(true);
        let target = {
            let mut timeline = Timeline::default();
            let image = Image::new("test.png".to_owned());
            let position = Position {
                inner: Box::new(image.clone()),
                position: Box::new(Vector2::new(100, 100))
            };
            timeline.append(
                TimelineElement::new(position),
                Duration::from_secs(0)..Duration::from_secs(1)
            );
            timeline.append(
                TimelineElement::new(image),
                Duration::from_secs(1)..Duration::from_secs(2)
            );
            timeline
        };

        while render_window.is_open() {
            let start = Instant::now();
            loop {
                let elapsed = Instant::now() - start;
                while let Some(event) = render_window.poll_event() {
                    if event == Event::Closed {
                        render_window.close();
                    }
                }
                let context = Context {
                    duration_from_start: elapsed.clone(),
                    duration_offset: elapsed,
                    render_states: RenderStates::default()
                };
                let frame_length = target.draw(&context, &mut render_texture);
                render_texture.display();
                {
                    render_window.clear(Color::TRANSPARENT);
                    let sprite = Sprite::with_texture(render_texture.texture());
                    render_window.draw(&sprite);
                    render_window.display();
                }
                match frame_length {
                    FrameLength::Consistent => {
                        sleep(Duration::new(1, 0) / 60);
                    },
                    FrameLength::Forever => {
                        sleep(Duration::new(60, 0))
                    },
                    FrameLength::Limited(duration) => {
                        sleep(duration)
                    },
                    FrameLength::End => {
                        break; //loop
                    }
                };
                render_texture.clear(Color::TRANSPARENT);
            }
        }
    }
}

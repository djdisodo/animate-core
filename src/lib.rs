#![feature(once_cell)]

mod element;

use std::any::Any;
use std::time::Duration;
use cgmath::num_traits::Num;
use opengl_graphics::GlGraphics;
use graphics::math::Matrix2d;
use dyn_clone::DynClone;
use std::fmt::Debug;
use serde_traitobject::*;
use serde_derive::*;

pub type Graphics = GlGraphics;

pub type BaseSigned = i32;
pub type BaseUnsigned = u32;

pub trait ValueProvider<T = BaseSigned>: DynClone + Debug + Serialize + Deserialize + 'static{
    fn get(&self, context: &Context) -> T;
}

impl<T: Copy + DynClone + Debug + Serialize + Deserialize + 'static> ValueProvider<T> for T {
    fn get(&self, context: &Context) -> T{
        *self
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
    transform: Matrix2d
}

#[cfg(test)]
fn main() {
    tests::main()
}

#[cfg(test)]
mod tests {
    use piston_window::{WindowSettings, PistonWindow, EventSettings, Events, RenderEvent, UpdateEvent};
    use graphics_buffer::RenderBuffer;
    use opengl_graphics::OpenGL;
    use crate::{Graphics, Context};
    use crate::element::{Image, Element};
    use std::path::Path;
    use graphics::Transformed;

    pub fn main() {
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let mut window: PistonWindow = WindowSettings::new("gl test", [500, 500])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut graphics = Graphics::new(opengl);

        let image = Image::new("test.png".to_owned());


        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                graphics.draw(args.viewport(), | c, gl | {
                    let mut context = Context {
                        duration_from_start: Default::default(),
                        duration_offset: Default::default(),
                        transform: c.transform
                    };
                    let size = image.get_size(&context);

                    image.draw(&context, gl)
                });
            }
        }
    }
}

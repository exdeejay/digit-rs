use crate::dwindow::Frame;

pub trait Anim {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn frames(&self) -> u32;
    fn fps(&self) -> u32;
    fn draw(&self, frame: u32, flipped: bool, buffer: &mut Frame);
}

mod manager;
pub use manager::{AnimHandle, AnimManager};

mod spritesheet;
pub use spritesheet::AnimSpritesheet;

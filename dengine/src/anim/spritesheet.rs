use super::{Anim, AnimHandle};
use crate::dwindow::Frame;
use image::{io::Reader as ImageReader, Pixel, RgbaImage};

pub struct AnimSpritesheet {
    spritesheet: RgbaImage,
    width: u32,
    height: u32,
    frames: u32,
    fps: u32,
}

impl AnimSpritesheet {
    pub fn from_handle(path: &str, handle: AnimHandle) -> AnimSpritesheet {
        let spritesheet = ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();

        if let AnimHandle {
            width: None,
            height: None,
            frames: None,
            fps: None,
            ..
        } = handle
        {
            let width = spritesheet.width();
            let height = spritesheet.height();
            AnimSpritesheet {
                spritesheet,
                width,
                height,
                frames: 1,
                fps: 1,
            }
        } else if let AnimHandle {
            width: Some(width),
            height: Some(height),
            frames: Some(frames),
            fps: Some(fps),
            ..
        } = handle
        {
            AnimSpritesheet {
                spritesheet,
                width,
                height,
                frames,
                fps,
            }
        } else {
            panic!("bad handle");
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32, frame: u32) -> &[u8] {
        if frame >= self.frames {
            panic!("Frame out of bounds");
        }
        if x >= self.width || y >= self.height {
            panic!("Coordinates out of bounds");
        }
        let (frame_x, frame_y) = (
            frame % (self.spritesheet.width() / self.width),
            frame / (self.spritesheet.width() / self.width()),
        );
        self.spritesheet
            .get_pixel(frame_x * self.width + x, frame_y * self.height + y)
            .channels()
    }
}

impl Anim for AnimSpritesheet {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn frames(&self) -> u32 {
        self.frames
    }
    fn fps(&self) -> u32 {
        self.fps
    }

    fn draw(&self, frame: u32, flipped: bool, buffer: &mut Frame) {
        for (x, y, pixel) in buffer
            .get_mut()
            .chunks_exact_mut(4)
            .enumerate()
            .map(|(i, pixel)| (i as u32 % self.width, i as u32 / self.width, pixel))
        {
            let x = match flipped {
                false => x,
                true => self.width - x - 1,
            };
            let anim_pixel = self.get_pixel(x, y, frame);
            pixel.copy_from_slice(anim_pixel);
        }
    }
}

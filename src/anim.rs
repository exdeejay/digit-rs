use image::{io::Reader as ImageReader, Pixel, RgbaImage};

pub struct Anim {
	width: u32,
	height: u32,
	frames: u32,
	fps: u32,
	spritesheet: RgbaImage,
}

impl Anim {
	pub fn new(filename: &str, width: u32, height: u32, frames: u32, fps: u32) -> Anim {
		let spritesheet = ImageReader::open(filename)
			.unwrap()
			.decode()
			.unwrap()
			.into_rgba8();
		Anim {
			width,
			height,
			frames,
			fps,
			spritesheet,
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
	pub fn frames(&self) -> u32 {
		self.frames
	}
	pub fn fps(&self) -> u32 {
		self.fps
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

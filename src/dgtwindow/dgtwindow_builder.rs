use super::DgtWindow;
use pixels::{Pixels, SurfaceTexture};
use std::{
	sync::{
		atomic::{AtomicI8, Ordering},
		mpsc, Arc, Mutex,
	},
	thread,
};
use winit::{
	dpi::{PhysicalPosition, PhysicalSize},
	event::Event,
	event_loop::{ControlFlow, EventLoop},
	platform::{run_return::EventLoopExtRunReturn, windows::EventLoopExtWindows},
	window::WindowBuilder,
};

pub struct DgtWindowBuilder {
	x: i32,
	y: i32,
	width: u32,
	height: u32,
	title: String,
	scale: f32,
}

pub struct FrameBuffer {
	buf: Vec<u8>,
	width: u32,
	height: u32,
	changed_size: bool,
}

impl FrameBuffer {
	pub fn new(width: u32, height: u32) -> FrameBuffer {
		let mut fb = FrameBuffer {
			buf: vec![],
			width: 0,
			height: 0,
			changed_size: false,
		};
		fb.set_size(width, height);
		fb.changed_size = false;
		fb
	}

	pub fn get_size(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	pub fn set_size(&mut self, width: u32, height: u32) {
		self.buf.resize((width * height * 4) as usize, 0);
		self.width = width;
		self.height = height;
		self.changed_size = true;
	}

	pub fn get(&self) -> &Vec<u8> {
		&self.buf
	}

	pub fn get_mut(&mut self) -> &mut Vec<u8> {
		&mut self.buf
	}
}

impl DgtWindowBuilder {
	pub fn new() -> Self {
		DgtWindowBuilder {
			x: 0,
			y: 0,
			width: 0,
			height: 0,
			title: String::from("Digit"),
			scale: 1.0,
		}
	}

	pub fn pos(mut self, x: i32, y: i32) -> Self {
		self.x = x;
		self.y = y;
		self
	}

	pub fn size(mut self, width: u32, height: u32) -> Self {
		self.width = width;
		self.height = height;
		self
	}

	pub fn scale(mut self, scale: f32) -> Self {
		self.scale = scale;
		self
	}

	pub fn title(mut self, title: &str) -> Self {
		self.title = String::from(title);
		self
	}

	pub fn build(self) -> DgtWindow {
		let scaled_width = self.width as f32 * self.scale;
		let scaled_height = self.height as f32 * self.scale;

		let (tx, rx) = mpsc::channel();
		thread::spawn(move || {
			let mut event_loop = EventLoop::<()>::new_any_thread();
			let window = Arc::new(
				WindowBuilder::new()
					.with_inner_size(PhysicalSize {
						width: scaled_width,
						height: scaled_height,
					})
					.with_title(self.title)
					.with_always_on_top(true)
					.with_transparent(true)
					.with_decorations(false)
					.build(&event_loop)
					.unwrap(),
			);
			window.set_outer_position(PhysicalPosition::new(self.x, self.y));

			let surf = SurfaceTexture::new(scaled_width as u32, scaled_width as u32, &*window);
			let mut pixels = Pixels::new(self.width, self.height, surf).unwrap();
			let buf0 = FrameBuffer::new(self.width, self.height);
			let buf1 = FrameBuffer::new(self.width, self.height);

			let current_buffer = Arc::new(AtomicI8::new(0));
			let buffers = Arc::new(vec![Mutex::new(buf0), Mutex::new(buf1)]);

			let dgtwindow = DgtWindow {
				x: self.x as f32,
				y: self.y as f32,
				window: window.clone(),
				current_buffer: current_buffer.clone(),
				buffers: buffers.clone(),
			};

			tx.send(dgtwindow).unwrap();

			let mut last_idx = 0;
			let scale = self.scale;
			let mut already_changed_size = false;
			event_loop.run_return(move |event, _, control_flow| {
				*control_flow = ControlFlow::Poll;
				match event {
					Event::MainEventsCleared => {
						let buf_idx = current_buffer.load(Ordering::SeqCst);
						if buf_idx != last_idx {
							let mut buf = buffers[(buf_idx as usize + 1) % 2].lock().unwrap();
							if !already_changed_size {
								if buf.changed_size {
									let scaled_width = (buf.width as f32 * scale) as u32;
									let scaled_height = (buf.height as f32 * scale) as u32;
									window.set_inner_size(PhysicalSize {
										width: scaled_width,
										height: scaled_height,
									});
									pixels.resize_surface(scaled_width, scaled_height);
									pixels.resize_buffer(buf.width, buf.height);
									buf.changed_size = false;
									already_changed_size = true;
								}
							} else {
								buf.changed_size = false;
								already_changed_size = false;
							}
							pixels.get_frame().copy_from_slice(buf.get());
							pixels.render().unwrap();
							last_idx = buf_idx;
						}
					}
					_ => (),
				}
			});
		});

		rx.recv().unwrap()
	}
}

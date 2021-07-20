use crate::anim::Anim;
use pixels::{Pixels, SurfaceTexture};
use std::{cell::RefCell, collections::HashMap, hash::Hash, ops::Deref, time::Instant};
use winit::{
	dpi::{PhysicalPosition, PhysicalSize},
	event::Event,
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
	window::{Window, WindowBuilder},
};

pub struct DgtWindow<T> {
	pub x: f32,
	pub y: f32,
	window: Window,
	scale: f32,
	anims: HashMap<T, Anim>,
	event_loop: RefCell<Option<EventLoop<()>>>,
	state: RefCell<WindowState<T>>,
}

struct WindowState<T> {
	current_anim: T,
	current_frame: u32,
	rendered: bool,
	last_frame_time: Instant,
	pixels: Pixels,
}

impl<T> DgtWindow<T>
where
	T: Eq + Hash + Copy,
{
	pub fn new(x: i32, y: i32, anims: HashMap<T, Anim>, start_anim: T, scale: f32) -> DgtWindow<T> {
		let event_loop = EventLoop::new();
		let width = anims.get(&start_anim).unwrap().width();
		let height = anims.get(&start_anim).unwrap().height();
		let scaled_width = (width as f32 * scale) as u32;
		let scaled_height = (height as f32 * scale) as u32;

		let window = WindowBuilder::new()
			.with_inner_size(PhysicalSize {
				width: scaled_width,
				height: scaled_height,
			})
			.with_title("Digit")
			.with_always_on_top(true)
			.with_transparent(true)
			.with_decorations(false)
			.build(&event_loop)
			.unwrap();
		window.set_outer_position(PhysicalPosition::new(x, y));
		let surf = SurfaceTexture::new(scaled_width, scaled_height, &window);
		let pixels = Pixels::new(width, height, surf).unwrap();

		DgtWindow {
			x: x as f32,
			y: y as f32,
			window,
			scale,
			anims,
			event_loop: RefCell::new(Some(event_loop)),
			state: RefCell::new(WindowState {
				current_anim: start_anim,
				current_frame: 0,
				rendered: false,
				last_frame_time: Instant::now(),
				pixels,
			}),
		}
	}

	pub fn update(&self) {
		self.set_outer_position(PhysicalPosition {
			x: self.x,
			y: self.y,
		});
		let mut evt_ref = self.event_loop.borrow_mut();
		if let Some(mut event_loop) = evt_ref.take() {
			event_loop.run_return(|event, _, control_flow| {
				*control_flow = ControlFlow::Exit;
				match event {
					Event::MainEventsCleared => {
						Self::render(self);
					}
					_ => (),
				}
			});
			*evt_ref = Some(event_loop);
		}
	}

	fn render(&self) {
		let mut state = self.state.borrow_mut();
		let current_frame = state.current_frame;
		let anim_type = state.current_anim;
		let anim = self.anims.get(&anim_type).unwrap();
		if !state.rendered {
			for (x, y, pixel) in state
				.pixels
				.get_frame()
				.chunks_exact_mut(4)
				.enumerate()
				.map(|(i, p)| (i as u32 % anim.width(), i as u32 / anim.width(), p))
			{
				let anim_pixel = anim.get_pixel(x, y, current_frame);
				pixel.copy_from_slice(anim_pixel);
			}
			state.pixels.render().unwrap();
			state.rendered = true;
		}

		if anim.frames() != 1
			&& state.last_frame_time.elapsed().as_millis() as u32 > 1000 / anim.fps()
		{
			state.current_frame += 1;
			if state.current_frame >= anim.frames() {
				state.current_frame = 0;
			}
			state.last_frame_time = Instant::now();
			state.rendered = false;
		}
	}

	pub fn set_anim(&self, new_anim: T) {
		let mut state = self.state.borrow_mut();
		let anim = self.anims.get(&new_anim).unwrap();
		self.window.set_inner_size(PhysicalSize {
			width: (anim.width() as f32 * self.scale) as u32,
			height: (anim.height() as f32 * self.scale) as u32,
		});
		let PhysicalSize {
			width: scaled_width,
			height: scaled_height,
		} = self.window.inner_size();
		state.pixels.resize_surface(scaled_width, scaled_height);
		state.pixels.resize_buffer(anim.width(), anim.height());
		state.current_anim = new_anim;
		state.current_frame = 0;
		state.rendered = false;
		state.last_frame_time = Instant::now();
	}
}

impl<T> Deref for DgtWindow<T> {
	type Target = Window;
	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

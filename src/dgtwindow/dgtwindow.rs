use super::{DgtWindowBuilder, FrameBuffer};
use std::{
	ops::Deref,
	sync::{
		atomic::{AtomicI8, Ordering},
		Arc, Mutex, MutexGuard,
	},
};
use winit::{dpi::PhysicalPosition, window::Window};

pub struct DgtWindow {
	pub x: f32,
	pub y: f32,
	pub window: Arc<Window>,
	pub current_buffer: Arc<AtomicI8>,
	pub buffers: Arc<Vec<Mutex<FrameBuffer>>>,
}

impl DgtWindow {
	pub fn new() -> DgtWindow {
		DgtWindowBuilder::new().build()
	}

	pub fn update(&self, _delta: f32) {
		self.set_outer_position(PhysicalPosition {
			x: self.x,
			y: self.y,
		});
	}

	pub fn get_buffer(&self) -> MutexGuard<'_, FrameBuffer> {
		let idx = self.current_buffer.load(Ordering::SeqCst);
		self.buffers[idx as usize].lock().unwrap()
	}

	pub fn swap_buffers(&self) {
		self.current_buffer.store(
			(self.current_buffer.load(Ordering::SeqCst) + 1) % 2,
			Ordering::SeqCst,
		);
	}
}

impl Deref for DgtWindow {
	type Target = Window;
	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

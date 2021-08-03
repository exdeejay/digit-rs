use crate::anim::AnimManager;
use crate::dgtwindow::{DgtWindow, DgtWindowBuilder};
use crate::dstate::IdleState;
use crate::fsm::StateMachine;
use std::mem::{self, MaybeUninit};
use winapi::{
	shared::windef::POINT,
	um::winuser::{
		GetMonitorInfoA, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONULL,
		MONITOR_DEFAULTTOPRIMARY,
	},
};

pub struct Digit {
	sm: Option<StateMachine<Digit>>,
	window: DgtWindow,
	anim_manager: AnimManager,
}

fn register_animations(anims: &mut AnimManager) {
	anims.register("idle").import("assets/idle.png");
	anims
		.register("walking")
		.width(64)
		.height(32)
		.frames(8)
		.fps(12)
		.import("assets/walking.png");
	anims.register("ready").import("assets/ready.png");
}

impl Digit {
	pub fn new() -> Digit {
		let mut anim_manager = AnimManager::new();
		register_animations(&mut anim_manager);

		let window = DgtWindowBuilder::new()
			.pos(32, get_taskbar_height())
			.size(32, 32)
			.scale(4.0)
			.title("Digit")
			.build();

		let mut digit = Digit {
			sm: None,
			window,
			anim_manager,
		};
		let sm = StateMachine::new();
		sm.init::<IdleState>(&mut digit);
		digit.sm = Some(sm);
		digit.window.swap_buffers();
		digit
	}

	pub fn update(&mut self, delta: f32) {
		if let Some(sm) = self.sm.take() {
			sm.update(self, delta);
			self.sm = Some(sm);
		} else {
			panic!("state machine gone (what the state machine doin)");
		}
		self.anim_manager.update(delta);
		self.window.update(delta);
	}

	pub fn render(&self) {
		let mut buffer = self.window.get_buffer();
		for byte in buffer.get_mut() {
			*byte = 0;
		}
		self.anim_manager.draw(&mut *buffer);
		for pixel in buffer.get_mut().chunks_exact_mut(4) {
			if pixel[3] == 0 {
				pixel[0] = 0;
				pixel[1] = 0;
				pixel[2] = 0;
			}
		}
		self.window.swap_buffers();
	}

	pub fn window(&self) -> &DgtWindow {
		&self.window
	}

	pub fn window_mut(&mut self) -> &mut DgtWindow {
		&mut self.window
	}

	pub fn set_anim(&self, name: &str) {
		self.anim_manager.set_anim(name);
	}

	pub fn set_flipped(&self, flipped: bool) {
		self.anim_manager.set_flipped(flipped);
	}
}

pub fn get_monitorinfo(x: i32, y: i32) -> Option<MONITORINFO> {
	unsafe {
		let hmonitor = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONULL);
		if hmonitor.is_null() {
			None
		} else {
			let mut mi = MaybeUninit::<MONITORINFO>::uninit().assume_init();
			mi.cbSize = mem::size_of::<MONITORINFO>() as u32;
			GetMonitorInfoA(hmonitor, &mut mi);
			Some(mi)
		}
	}
}

fn get_taskbar_height() -> i32 {
	unsafe {
		let hmonitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
		let mut mi = MaybeUninit::<MONITORINFO>::uninit().assume_init();
		mi.cbSize = mem::size_of::<MONITORINFO>() as u32;
		GetMonitorInfoA(hmonitor, &mut mi);
		mi.rcWork.bottom - 128
	}
}

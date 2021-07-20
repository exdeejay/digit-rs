use crate::anim::Anim;
use crate::dstate::IdleState;
use crate::fsm::StateMachine;
use crate::window::DgtWindow;
use std::collections::HashMap;
use std::mem::{self, MaybeUninit};
use winapi::{
	shared::windef::HMONITOR,
	um::winuser::{GetMonitorInfoA, MONITORINFO},
};
use winit::platform::windows::MonitorHandleExtWindows;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum AnimType {
	Idle,
	Walk,
}

pub struct Digit {
	anim_state: Option<StateMachine<Digit>>,
	pub window: DgtWindow<AnimType>,
}

impl Digit {
	pub fn new() -> Digit {
		let mut anims = HashMap::new();
		anims.insert(AnimType::Idle, Anim::new("assets/idle.png", 32, 32, 1, 0));
		anims.insert(AnimType::Walk, Anim::new("assets/walk.png", 64, 32, 4, 6));
		let anims = anims;

		let mut window = DgtWindow::new(0, 0, anims, AnimType::Idle, 4.0);
		window.x = 32.0;
		window.y = match window.primary_monitor() {
			Some(mon) => unsafe {
				let mut mi = MaybeUninit::<MONITORINFO>::uninit().assume_init();
				mi.cbSize = mem::size_of::<MONITORINFO>() as u32;
				GetMonitorInfoA(mon.hmonitor() as HMONITOR, &mut mi);
				mi.rcWork.bottom as f32 - 128.0
			},
			None => 0.0,
		};
		let mut digit = Digit {
			anim_state: None,
			window,
		};
		digit.anim_state = Some(StateMachine::new::<IdleState>(&digit));
		digit
	}

	pub fn update(&mut self) {
		self.anim_state.as_ref().unwrap().update(self);
		self.window.update();
	}
}

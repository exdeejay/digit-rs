use crate::{
	digit::{AnimType, Digit},
	dstate::{DState, WalkState},
	fsm::StateMachine,
};
use std::time::Instant;

pub struct IdleState {
	start: Instant,
	duration: f32,
}

impl DState<Digit> for IdleState {
	fn enter(_sm: &StateMachine<Digit>, client: &Digit) -> Self {
		println!("idle");
		client.window.set_anim(AnimType::Idle);
		Self {
			start: Instant::now(),
			duration: rand::random::<f32>() * 1.0 + 1.0,
		}
	}
	fn update(self: Box<Self>, sm: &StateMachine<Digit>, client: &Digit) -> Box<dyn DState<Digit>> {
		if self.start.elapsed().as_millis() as f32 / 1000.0 > self.duration {
			sm.transit::<WalkState>(client)
		} else {
			self
		}
	}
}

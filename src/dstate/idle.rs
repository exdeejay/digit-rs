use crate::{
	digit::Digit,
	dstate::{DState, WalkState},
	fsm::StateMachine,
};
use std::time::Instant;

pub struct IdleState {
	start: Instant,
	duration: f32,
}

impl DState<Digit> for IdleState {
	fn enter(_sm: &StateMachine<Digit>, digit: &mut Digit) -> Box<dyn DState<Digit>> {
		digit.set_anim("idle");
		Box::new(Self {
			start: Instant::now(),
			duration: rand::random::<f32>() * 1.0 + 1.0,
		})
	}
	fn update(
		self: Box<Self>,
		sm: &StateMachine<Digit>,
		digit: &mut Digit,
		_delta: f32,
	) -> Box<dyn DState<Digit>> {
		if self.start.elapsed().as_millis() as f32 / 1000.0 > self.duration {
			sm.transit::<WalkState>(digit)
		} else {
			self
		}
	}
}

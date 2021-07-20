use crate::{
	digit::{AnimType, Digit},
	dstate::DState,
	fsm::StateMachine,
};

pub struct WalkState {}

impl DState<Digit> for WalkState {
	fn enter(_sm: &StateMachine<Digit>, client: &Digit) -> Self {
		println!("walk");
		client.window.set_anim(AnimType::Walk);
		Self {}
	}
	fn update(
		self: Box<Self>,
		_sm: &StateMachine<Digit>,
		_client: &Digit,
	) -> Box<dyn DState<Digit>> {
		self
	}
}

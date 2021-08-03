use crate::{
	digit::{get_monitorinfo, Digit},
	dstate::{DState, IdleState},
	fsm::StateMachine,
};

pub struct WalkState {
	walking_right: bool,
	destination: i32,
}

const SPEED: f32 = 300.0;

impl DState<Digit> for WalkState {
	fn enter(sm: &StateMachine<Digit>, digit: &mut Digit) -> Box<dyn DState<Digit>> {
		let mi_opt = get_monitorinfo(digit.window().x as i32, digit.window().y as i32);
		if let Some(mi) = mi_opt {
			let length = mi.rcWork.right - mi.rcWork.left;
			let destination = (rand::random::<f32>() * length as f32) as i32 + mi.rcWork.left;
			let walking_right = digit.window().x < destination as f32;
			digit.set_anim("walking");
			digit.set_flipped(!walking_right);
			Box::new(Self {
				walking_right,
				destination,
			})
		} else {
			sm.transit::<IdleState>(digit)
		}
	}
	fn update(
		self: Box<Self>,
		sm: &StateMachine<Digit>,
		digit: &mut Digit,
		delta: f32,
	) -> Box<dyn DState<Digit>> {
		// positive if dest is to the right
		// negative if dest is to the left
		let direction = match self.walking_right {
			true => 1.0,
			false => -1.0,
		};

		digit.window_mut().x += direction * delta * SPEED;

		if (self.destination as f32 - digit.window().x).signum() != direction {
			digit.window_mut().x = self.destination as f32;
			sm.transit::<IdleState>(digit)
		} else {
			self
		}
	}
}

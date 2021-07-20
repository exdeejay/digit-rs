mod anim;
mod digit;
mod dstate;
mod fsm;
mod window;

use digit::Digit;

pub fn run() {
	let mut digit = Digit::new();
	loop {
		digit.update();
	}
}

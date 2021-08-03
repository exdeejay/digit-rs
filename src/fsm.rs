use crate::dstate::DState;
use std::cell::RefCell;

pub struct StateMachine<T> {
	current: RefCell<Option<Box<dyn DState<T>>>>,
}

impl<T> StateMachine<T> {
	pub fn new() -> StateMachine<T> {
		StateMachine {
			current: RefCell::new(None),
		}
	}

	pub fn init<S: 'static + DState<T>>(&self, client: &mut T) {
		*self.current.borrow_mut() = Some(self.transit::<S>(client));
	}

	pub fn transit<S: 'static + DState<T>>(&self, client: &mut T) -> Box<dyn DState<T>> {
		S::enter(self, client)
	}

	pub fn update(&self, client: &mut T, delta: f32) {
		let state = self.current.borrow_mut().take().unwrap();
		let new = state.update(self, client, delta);
		*self.current.borrow_mut() = Some(new);
	}
}

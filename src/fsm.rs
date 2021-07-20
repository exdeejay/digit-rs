use crate::dstate::DState;
use std::cell::RefCell;

pub struct StateMachine<T> {
	current: RefCell<Option<Box<dyn DState<T>>>>,
}

impl<T> StateMachine<T> {
	pub fn new<S: 'static + DState<T>>(client: &T) -> StateMachine<T> {
		let sm = StateMachine {
			current: RefCell::new(None),
		};
		*sm.current.borrow_mut() = Some(Box::new(S::enter(&sm, client)));
		sm
	}

	pub fn transit<C: 'static + DState<T>>(&self, client: &T) -> Box<C> {
		Box::new(C::enter(self, client))
	}

	pub fn update(&self, client: &T) {
		let state = self.current.borrow_mut().take().unwrap();
		let new = state.update(self, client);
		*self.current.borrow_mut() = Some(new);
	}
}

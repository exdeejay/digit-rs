mod idle;
pub use idle::IdleState;
mod walk;
pub use walk::WalkState;

use crate::fsm::StateMachine;

pub trait DState<T> {
	fn enter(sm: &StateMachine<T>, client: &T) -> Self
	where
		Self: Sized;
	fn update(self: Box<Self>, sm: &StateMachine<T>, client: &T) -> Box<dyn DState<T>>;
}

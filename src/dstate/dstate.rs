use crate::fsm::StateMachine;

pub trait DState<T> {
	fn enter(sm: &StateMachine<T>, client: &mut T) -> Box<dyn DState<T>>
	where
		Self: Sized;
	fn update(self: Box<Self>, sm: &StateMachine<T>, client: &mut T, delta: f32) -> Box<dyn DState<T>>;
}

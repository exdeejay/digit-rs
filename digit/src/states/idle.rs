use super::WalkState;
use crate::Digit;
use dengine::fsm::{DState, StateMachine};
use std::time::Instant;

pub struct IdleState {
    start: Instant,
    duration: f32,
    is_dancing: bool,
}

impl DState<Digit> for IdleState {
    fn enter(_sm: &StateMachine<Digit>, digit: &mut Digit) -> Box<dyn DState<Digit>> {
        digit.anims().set_anim("ready");
        Box::new(Self {
            start: Instant::now(),
            duration: rand::random::<f32>() * 10.0 + 1.0,
            is_dancing: false,
        })
    }
    fn update(
        mut self: Box<Self>,
        sm: &StateMachine<Digit>,
        digit: &mut Digit,
        _delta: f32,
    ) -> Box<dyn DState<Digit>> {
        if self.start.elapsed().as_millis() as f32 / 1000.0 > self.duration {
            sm.transit::<WalkState>(digit)
        } else {
        let is_now_dancing = digit.is_dancing();
        if is_now_dancing != self.is_dancing {
            self.is_dancing = is_now_dancing;
            if is_now_dancing {
                digit.anims().set_anim("dancing");
            } else {
                digit.anims().set_anim("ready");
            }
        }
        self
        }
    }
}

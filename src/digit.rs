use crate::states::IdleState;
use dengine::{
    anim::AnimManager,
    dwindow::{DWindow, DWindowBuilder},
    fsm::StateMachine,
};

pub struct Digit {
    sm: Option<StateMachine<Digit>>,
    window: DWindow,
    anim_manager: AnimManager,
}

fn register_animations(anims: &mut AnimManager) {
    anims.register("idle").import("assets/idle.png");
    anims
        .register("walking")
        .width(64)
        .height(32)
        .frames(8)
        .fps(12)
        .import("assets/walking.png");
    anims
        .register("dancing")
        .width(32)
        .height(32)
        .frames(8)
        .fps(12)
        .import("assets/wagging.png");
    anims.register("ready").import("assets/ready.png");
}

impl Digit {
    pub fn new() -> Digit {
        let mut anim_manager = AnimManager::new();
        register_animations(&mut anim_manager);

        let height = 32.0;
        let scale = 4.0;
        let window = DWindowBuilder::new()
            .pos(32, crate::get_taskbar_height() - (height * scale) as i32)
            .size(32, 32)
            .scale(scale)
            .title("Digit")
            .build();

        let mut digit = Digit {
            sm: None,
            window,
            anim_manager,
        };
        let sm = StateMachine::new();
        sm.init::<IdleState>(&mut digit);
        digit.sm = Some(sm);
        digit.window.swap_buffers();
        digit
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(sm) = self.sm.take() {
            sm.update(self, delta);
            self.sm = Some(sm);
        } else {
            panic!("state machine gone (what the state machine doin)");
        }
        self.anim_manager.update(delta);
        self.window.update(delta);
    }

    pub fn render(&self) {
        let mut buffer = self.window.get_buffer();
        for byte in buffer.get_mut() {
            *byte = 0;
        }
        self.anim_manager.draw(&mut *buffer);
        for pixel in buffer.get_mut().chunks_exact_mut(4) {
            if pixel[3] == 0 {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }
        }
        self.window.swap_buffers();
    }

    pub fn window(&self) -> &DWindow {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut DWindow {
        &mut self.window
    }

    pub fn anims(&self) -> &AnimManager {
        &self.anim_manager
    }
}

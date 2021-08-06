use crate::{
    anim::{Anim, AnimSpritesheet},
    dwindow::Frame,
};
use std::{cell::RefCell, collections::HashMap, default::Default, rc::Rc};

struct AnimState {
    pub current_anim: Option<Rc<Box<dyn Anim>>>,
    frame: u32,
    elapsed: f32,
    flipped: bool,
}

pub struct AnimManager {
    anims: HashMap<String, Rc<Box<dyn Anim>>>,
    state: RefCell<AnimState>,
}

#[derive(Default)]
pub struct AnimHandle<'a> {
    pub name: String,
    pub manager: Option<&'a mut AnimManager>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frames: Option<u32>,
    pub fps: Option<u32>,
}

impl<'a> AnimHandle<'a> {
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn frames(mut self, frames: u32) -> Self {
        self.frames = Some(frames);
        self
    }

    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = Some(fps);
        self
    }

    pub fn import(mut self, path: &str) {
        if let Some(manager) = self.manager.take() {
            manager.register_file_handle(path, self);
        } else {
            panic!("bad handle");
        }
    }
}

impl AnimManager {
    pub fn new() -> AnimManager {
        AnimManager {
            anims: HashMap::new(),
            state: RefCell::new(AnimState {
                current_anim: None,
                frame: 0,
                elapsed: 0.0,
                flipped: false,
            }),
        }
    }

    pub fn register(&mut self, name: &str) -> AnimHandle {
        AnimHandle {
            name: String::from(name),
            manager: Some(self),
            ..Default::default()
        }
    }

    fn register_file_handle(&mut self, path: &str, handle: AnimHandle) {
        let name = handle.name.clone();
        let new_anim = AnimSpritesheet::from_handle(path, handle);
        self.anims.insert(name, Rc::new(Box::new(new_anim)));
    }

    pub fn set_anim(&self, name: &str) {
        let mut state = self.state.borrow_mut();
        state.frame = 0;
        if let Some(anim) = self.anims.get(name) {
            state.current_anim = Some(Rc::clone(anim));
        } else {
            panic!("invalid anim name");
        }
    }

    pub fn set_flipped(&self, flipped: bool) {
        let mut state = self.state.borrow_mut();
        state.flipped = flipped;
    }

    pub fn update(&self, delta: f32) {
        let mut state = self.state.borrow_mut();
        state.elapsed += delta;
        if let Some(anim) = &state.current_anim {
            let frame_count = anim.frames();
            if state.elapsed > 1.0 / anim.fps() as f32 {
                state.elapsed = 0.0;
                state.frame = (state.frame + 1) % frame_count;
            }
        }
    }

    pub fn draw(&self, buffer: &mut Frame) {
        let state = self.state.borrow();
        if let Some(anim) = &state.current_anim {
            if buffer.size() != (anim.width(), anim.height()) {
                buffer.set_size(anim.width(), anim.height());
            }
            anim.draw(state.frame, state.flipped, buffer);
        }
    }
}

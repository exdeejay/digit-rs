use crate::{
    services::media::MediaPlaybackStatus,
    services::{media::MediaSession, Services},
    states::IdleState,
};
use dengine::{
    anim::AnimManager,
    dwindow::{DWindow, DWindowBuilder, FrameBuffer},
    fsm::StateMachine,
};
use pixels::Pixels;
use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};

pub struct Digit {
    sm: Option<StateMachine<Digit>>,
    window: DWindow,
    anim_manager: AnimManager,
    dancing: Arc<AtomicBool>,
}

enum DigitWindowEvent {
    Click
}

impl Digit {
    pub fn new() -> Digit {
        let mut anim_manager = AnimManager::new();
        register_animations(&mut anim_manager);

        let height = 32.0;
        let scale = 4.0;
        let window = DWindowBuilder::<DigitWindowEvent>::new()
            .pos(32, crate::get_taskbar_height() - (height * scale) as i32)
            .size(32, 32)
            .scale(scale)
            .title("Digit")
            .loop_fn(render_loop)
            .build();

        let mut digit = Digit {
            sm: None,
            window,
            anim_manager,
            dancing: Arc::new(AtomicBool::new(false)),
        };
        register_media_callback(&digit.dancing);
        let sm = StateMachine::new();
        sm.init::<IdleState>(&mut digit);
        digit.sm = Some(sm);

        digit.window.swap_buffers();
        digit
    }

    pub fn is_dancing(&self) -> bool {
        self.dancing.load(Ordering::SeqCst)
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
        let mut frame = self.window.framebuffer().get_back_buffer();
        for byte in frame.get_mut().deref_mut() {
            *byte = 0;
        }
        self.anim_manager.draw(&mut frame);
        for pixel in frame.get_mut().chunks_exact_mut(4) {
            if pixel[3] == 0 {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }
        }
        drop(frame);
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

fn render_loop(
    scale: f32,
    framebuffer: &FrameBuffer,
    pixels: Pixels,
    window: &Window,
    event_loop: EventLoop<DigitWindowEvent>,
) {
    let PhysicalSize {
        mut width,
        mut height,
    } = window.inner_size();

    let mut event_loop = event_loop;
    let mut pixels = pixels;
    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                let frame = framebuffer.get_front_buffer();

                if frame.width != width || frame.height != height {
                    width = frame.width;
                    height = frame.height;
                    let scaled_width = (frame.width as f32 * scale) as u32;
                    let scaled_height = (frame.height as f32 * scale) as u32;
                    window.set_inner_size(PhysicalSize {
                        width: scaled_width,
                        height: scaled_height,
                    });
                    pixels.resize_surface(scaled_width, scaled_height);
                    pixels.resize_buffer(frame.width, frame.height);
                }

                pixels.get_frame().copy_from_slice(&*frame.buffer);
                drop(frame);

                pixels.render().unwrap();
            }
            _ => (),
        }
    });
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

fn register_media_callback(dancing_bool: &Arc<AtomicBool>) {
    let dancing_ref = Arc::downgrade(dancing_bool);
    let mut media_service = Services::media();
    let callback = move |session_opt: &Option<MediaSession>| {
        if let Some(dancing) = dancing_ref.upgrade() {
            match session_opt {
                Some(session) => {
                    let should_dance = session.GetPlaybackInfo().unwrap().PlaybackStatus().unwrap()
                        == MediaPlaybackStatus::Playing;
                    dancing.store(should_dance, Ordering::SeqCst);
                }
                None => dancing.store(false, Ordering::SeqCst),
            }
        }
    };
    callback(&Some(media_service.get_media_session()));
    media_service.subscribe(callback);
}

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

/**
 * Custom user window events
 * Exists here to be pluggable into DWindow
 */
enum DigitWindowEvent {
    Click,
}

/**
 * Main game class
 * Only one instance of this class is supported currently
 */
pub struct Digit {
    sm: Option<StateMachine<Digit>>,
    window: DWindow,
    anim_manager: AnimManager,
    dancing: Arc<AtomicBool>,
}

impl Digit {
    pub fn new() -> Digit {
        // Mark process as DPI aware so that the OS reports correct
        // monitor size even when scaled
        crate::set_process_dpi_aware();

        // Initialize and register all animations
        let mut anim_manager = AnimManager::new();
        register_animations(&mut anim_manager);

        // Create window with event handler `render_loop()`
        let height = 32.0;
        let scale = 4.0;
        let window = DWindowBuilder::<DigitWindowEvent>::new()
            .pos(32, crate::get_taskbar_height() - (height * scale) as i32)
            .size(32, 32)
            .scale(scale)
            .title("Digit")
            .loop_fn(render_loop)
            .build();

        // State machine must start out as None to be initialized later
        let mut digit = Digit {
            sm: None,
            window,
            anim_manager,
            dancing: Arc::new(AtomicBool::new(false)),
        };
        // Sync dancing variable with media status on OS
        register_media_callback(&digit.dancing);

        // Construct state machine and run initialization with beginning state
        // This is why we needed the state machine to be None, so that the first
        // state can access the data struct without having a separate init method
        let sm = StateMachine::new();
        sm.init::<IdleState>(&mut digit);
        digit.sm = Some(sm);

        // Render first frame
        digit.window.swap_buffers();
        digit
    }

    pub fn is_dancing(&self) -> bool {
        self.dancing.load(Ordering::SeqCst)
    }

    /**
     * Update function to be run every frame
     */
    pub fn update(&mut self, delta: f32) {
        // Update state machine
        if let Some(sm) = self.sm.take() {
            sm.update(self, delta);
            self.sm = Some(sm);
        } else {
            panic!("state machine gone (what the state machine doin)");
        }
        // Update animation
        self.anim_manager.update(delta);
        // Update window
        self.window.update(delta);
    }

    /**
     * Render function to be run every frame (?)
     * Maybe could cut back on when it runs to optimize CPU usage
     */
    pub fn render(&self) {
        // Lock back buffer that contains the previous frame
        let mut frame = self.window.framebuffer().get_back_buffer();
        // Zero out previous frame
        for byte in frame.get_mut().deref_mut() {
            *byte = 0;
        }
        // Render current animation on window frame
        self.anim_manager.draw(&mut frame);
        // Make transparent pixels fully black just in case
        for pixel in frame.get_mut().chunks_exact_mut(4) {
            if pixel[3] == 0 {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }
        }
        // Drop frame manually to unlock it and enable swapping frame buffers
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

/**
 * List of animations to register in a declarative manner
 */
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

/**
 * Register a variable to be synced to be true when media is playing
 */
fn register_media_callback(dancing_bool: &Arc<AtomicBool>) {
    // Downgrade to a weak ref so that the callback can exist forever, even if the
    // reference gets invalidated or dropped
    let dancing_ref = Arc::downgrade(dancing_bool);

    // Construct callback to be called when media status changes
    // Just updates dancing_bool
    let callback = move |session_opt: &Option<MediaSession>| {
        if let Some(dancing) = dancing_ref.upgrade() {
            match session_opt {
                Some(session) => {
                    // Set dancing_bool to true if media is playing
                    // We can also detect if this is a video if we want
                    let should_dance = session.GetPlaybackInfo().unwrap().PlaybackStatus().unwrap()
                        == MediaPlaybackStatus::Playing;
                    dancing.store(should_dance, Ordering::SeqCst);
                }

                // Fails closed so if there was an error getting the session,
                // don't update the variable
                None => dancing.store(false, Ordering::SeqCst),
            }
        }
    };

    // Lock media service
    let mut media_service = Services::media();
    // Run callback immediately to correct internal state
    callback(&media_service.get_media_session());
    // And then register it to be called whenever the service changes
    media_service.subscribe(callback);
}

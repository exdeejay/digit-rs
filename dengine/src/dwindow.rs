use parking_lot::{Mutex, MutexGuard};
use pixels::{raw_window_handle::HasRawWindowHandle, Pixels, SurfaceTexture};
use raw_window_handle::RawWindowHandle;
use std::{
    marker::PhantomData,
    mem,
    ops::Deref,
    sync::{mpsc, Arc},
    thread,
};
use winapi::um::winuser::{SetWindowLongA, GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    platform::windows::EventLoopExtWindows,
    window::{Window, WindowBuilder},
};

pub struct DWindow {
    pub x: f32,
    pub y: f32,
    scale: f32,
    window: Arc<Window>,
    framebuffer: Arc<FrameBuffer>,
}

impl DWindow {
    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn update(&self, _delta: f32) {
        self.set_outer_position(PhysicalPosition {
            x: self.x - (self.x % self.scale),
            y: self.y - (self.y % self.scale),
        });
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        &self.framebuffer
    }

    pub fn swap_buffers(&self) {
        self.framebuffer.swap_buffers();
    }
}

impl Deref for DWindow {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl Frame {
    fn new(width: u32, height: u32) -> Frame {
        let mut buffer = Vec::new();
        buffer.resize((width * height * 4) as usize, 0);
        Frame {
            width,
            height,
            buffer,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.buffer.resize((width * height * 4) as usize, 0);
        self.width = width;
        self.height = height;
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }
}

pub struct FrameBuffer {
    front_buffer: Mutex<Box<Frame>>,
    back_buffer: Mutex<Box<Frame>>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        FrameBuffer {
            front_buffer: Mutex::new(Box::new(Frame::new(width, height))),
            back_buffer: Mutex::new(Box::new(Frame::new(width, height))),
        }
    }

    pub fn get_back_buffer(&self) -> MutexGuard<'_, Box<Frame>> {
        self.back_buffer.try_lock().unwrap()
    }

    pub fn get_front_buffer(&self) -> MutexGuard<'_, Box<Frame>> {
        self.front_buffer.lock()
    }

    pub fn swap_buffers(&self) {
        let mut fb = self.front_buffer.lock();
        let mut bb = self.back_buffer.try_lock().unwrap();
        mem::swap(fb.as_mut(), bb.as_mut());
    }
}

pub struct DWindowBuilder<T>
where
    T: 'static + Send,
{
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    title: String,
    scale: f32,
    loop_fn: Option<Box<dyn Fn(f32, &FrameBuffer, Pixels, &Window, EventLoop<T>) + 'static + Send>>,
    event_type: PhantomData<T>,
}

impl<T> DWindowBuilder<T>
where
    T: 'static + Send,
{
    pub fn new() -> Self {
        DWindowBuilder {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            title: String::from("Digit"),
            scale: 1.0,
            loop_fn: None,
            event_type: PhantomData::<T>,
        }
    }

    pub fn pos(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn loop_fn<F>(mut self, loop_fn: F) -> Self
    where
        F: Fn(f32, &FrameBuffer, Pixels, &Window, EventLoop<T>) + 'static + Send,
    {
        self.loop_fn = Some(Box::new(loop_fn));
        self
    }

    pub fn build(mut self) -> DWindow {
        let scaled_width = (self.width as f32 * self.scale) as u32;
        let scaled_height = (self.height as f32 * self.scale) as u32;

        let loop_fn = self.loop_fn.take().unwrap();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let event_loop = EventLoop::<T>::new_any_thread();
            let window = Arc::new(
                WindowBuilder::new()
                    .with_inner_size(PhysicalSize {
                        width: scaled_width,
                        height: scaled_height,
                    })
                    .with_title(self.title)
                    .with_always_on_top(true)
                    .with_transparent(true)
                    .with_decorations(false)
                    .build(&event_loop)
                    .unwrap(),
            );
            window.set_outer_position(PhysicalPosition::new(self.x, self.y));
            if let RawWindowHandle::Windows(handle) = window.raw_window_handle() {
                unsafe {
                    SetWindowLongA(
                        mem::transmute(handle.hwnd),
                        GWL_EXSTYLE,
                        (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE) as i32,
                    );
                }
            }

            let surf = SurfaceTexture::new(scaled_width as u32, scaled_width as u32, &*window);
            let pixels = Pixels::new(self.width, self.height, surf).unwrap();
            let dwindow = DWindow {
                x: self.x as f32,
                y: self.y as f32,
                scale: self.scale,
                window,
                framebuffer: Arc::new(FrameBuffer::new(self.width, self.height)),
            };

            let framebuffer = dwindow.framebuffer.clone();
            let window = dwindow.window.clone();

            tx.send(dwindow).unwrap();

            loop_fn(self.scale, &framebuffer, pixels, &window, event_loop);
        });

        rx.recv().unwrap()
    }
}

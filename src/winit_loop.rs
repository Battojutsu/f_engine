use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
#[derive(Default)]
struct App {
    window: Option<Window>,
    frames: Arc<Mutex<i32>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("exiting");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut m = self.frames.lock().unwrap();
                *m += 1;
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    let frame_counter = Mutex::new(0);

    let a_frame_counter: Arc<Mutex<i32>> = Arc::new(frame_counter);
    let a_frame_counter: Arc<Mutex<i32>> = Arc::clone(&a_frame_counter);
    app.frames =  Arc::clone(&a_frame_counter);

    thread::spawn(move || {
        loop {
            sleep(Duration::from_secs(1));
            let mut frames = a_frame_counter.lock().unwrap();
            println!("Frames{}", *frames);
            *frames = 0;
        }
    });

    match event_loop.run_app(&mut app) {
        Err(why) => panic!("{:?}", why),
        Ok(v) => {
            return v;
        }
    };

}

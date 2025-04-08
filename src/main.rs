use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    // Flag used to signal all threads to stop working
    let running = Arc::new(AtomicBool::new(true));

    // Clone the flag for the signal handler
    let r = Arc::clone(&running);
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Start camera thread
    let r = Arc::clone(&running);
    let camera_thread = thread::spawn(move || {
        let cam = open_camera();

        while r.load(Ordering::SeqCst) {
            println!("Camera thread is working...");
            thread::sleep(Duration::from_secs(1));
        }

        close_camera(cam);
        println!("Camera thread exited cleanly.");
    });

    camera_thread.join().expect("Camera thread panicked!");
}

struct Camera {}
impl Camera {
    fn new() -> Self {
        Self {}
    }
}

fn close_camera(_cam: Camera) {
    // TODO: Implement camera closing logic
}

fn open_camera() -> Camera {
    // TODO: Implement camera opening logic
    Camera::new()
}

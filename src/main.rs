use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    // Flag used to signal all threads to stop working
    let running = Arc::new(AtomicBool::new(true));

    set_ctrlc_handler(&running);

    let camera_thread = start_camera_thread(&running);

    camera_thread.join().expect("Camera thread panicked!");
}

fn set_ctrlc_handler(running: &Arc<AtomicBool>) {
    let r = Arc::clone(running);
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
}

fn start_camera_thread(running: &Arc<AtomicBool>) -> thread::JoinHandle<()> {
    // Start camera thread
    let r = Arc::clone(running);
    let camera_thread = thread::spawn(move || {
        let cam = open_camera();

        while r.load(Ordering::SeqCst) {
            println!("Camera thread is working...");
            thread::sleep(Duration::from_secs(1));
        }

        close_camera(cam);
        println!("Camera thread exited cleanly.");
    });
    camera_thread
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

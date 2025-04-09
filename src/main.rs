use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const RING_BUFFER_SIZE: usize = 10;

fn main() {
    // Flag used to signal all threads to stop working
    let running = Arc::new(AtomicBool::new(true));

    set_ctrlc_handler(Arc::clone(&running));

    // Dummy ring buffer
    let ring_buffer = Arc::new(Mutex::new(VecDeque::<Image>::with_capacity(
        RING_BUFFER_SIZE,
    )));

    let camera_thread = start_camera_thread(Arc::clone(&running), Arc::clone(&ring_buffer));
    camera_thread.join().expect("Camera thread panicked!");
}

fn set_ctrlc_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
}

fn start_camera_thread(
    running: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<VecDeque<Image>>>,
) -> thread::JoinHandle<()> {
    let camera_thread = thread::spawn(move || {
        let cam = open_camera();

        while running.load(Ordering::SeqCst) {
            println!("Camera thread is working...");
            {
                let mut b = ring_buffer.lock().unwrap();
                if b.len() >= RING_BUFFER_SIZE {
                    println!("Popping image off buffer");
                    b.pop_front();
                }
                println!("Pushing image onto buffer");
                b.push_back(Image::new());
            }
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

struct Image {}
impl Image {
    fn new() -> Self {
        Self {}
    }
}

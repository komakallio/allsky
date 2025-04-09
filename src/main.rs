use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let cameras = libtoupcam::enumerate_cameras();
    println!("Found {} cameras", cameras.len());
    cameras.iter().for_each(|camera| {
        println!("{:?}", camera);
    });

    // Flag used to signal all threads to stop working
    let running = Arc::new(AtomicBool::new(true));

    set_ctrlc_handler(Arc::clone(&running));

    // Dummy ring buffer
    let ring_buffer = Arc::new(Mutex::new(RingBuffer::new(10)));

    let camera_thread = start_camera_thread(Arc::clone(&running), Arc::clone(&ring_buffer));

    let analysis_thread = start_analysis_thread(Arc::clone(&running), Arc::clone(&ring_buffer));

    camera_thread.join().expect("Camera thread panicked!");
    analysis_thread.join().expect("Analysis thread panicked!");
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
    ring_buffer: Arc<Mutex<RingBuffer<Image>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let cam = open_camera();

        while running.load(Ordering::SeqCst) {
            {
                println!("Camera thread is working...");
                let mut b = ring_buffer.lock().unwrap();
                b.add(Image::new());
            }
            thread::sleep(Duration::from_millis(250));
        }

        close_camera(cam);
        println!("Camera thread exited cleanly.");
    })
}

fn start_analysis_thread(
    running: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<RingBuffer<Image>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(5));
            {
                println!("Running analysis...");
                let mut b = ring_buffer.lock().unwrap();
                println!("{:?}", b.get_contents());
            }
        }
        println!("Analysis thread exited cleanly.");
    })
}

struct RingBuffer<T> {
    internal_queue: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            internal_queue: VecDeque::<T>::with_capacity(capacity),
            capacity,
        }
    }

    fn add(&mut self, item: T) {
        if self.internal_queue.len() >= self.capacity {
            _ = self.internal_queue.pop_front();
        }
        self.internal_queue.push_back(item);
    }

    fn get_contents(&mut self) -> &[T] {
        self.internal_queue.make_contiguous()
    }
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

#[derive(Debug)]
struct Image {}
impl Image {
    fn new() -> Self {
        Self {}
    }
}

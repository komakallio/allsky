use crate::image::Image;
use crate::ring_buffer::RingBuffer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_analysis_thread(
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

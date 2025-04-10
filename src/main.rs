use crate::analysis::start_analysis_thread;
use crate::ctrlc_handler::set_ctrlc_handler;
use crate::image_capture::start_camera_thread;
use crate::ring_buffer::RingBuffer;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

mod analysis;
mod camera;
mod ctrlc_handler;
mod image;
mod image_capture;
mod ring_buffer;

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

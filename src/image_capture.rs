use crate::camera::{Camera, toupcam};
use crate::image::Image;
use crate::ring_buffer::RingBuffer;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, atomic::Ordering};
use std::thread;
use std::time::Duration;

pub(crate) fn start_camera_thread(
    running: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<RingBuffer<Image>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut found_devices = toupcam::ToupcamDevice::enumerate_devices();
        found_devices.iter().for_each(|device| {
            println!(
                "Found camera: {} (ID: {})",
                device.get_name(),
                device.get_id()
            );
        });
        println!("Picking first camera");
        let cam = found_devices.first_mut().expect("No cameras found!");
        cam.open();

        while running.load(Ordering::SeqCst) {
            {
                println!("Camera thread is working...");
                let mut b = ring_buffer.lock().unwrap();
                b.add(Image::new());
            }
            thread::sleep(Duration::from_millis(250));
        }

        cam.close();
        println!("Camera thread exited cleanly.");
    })
}

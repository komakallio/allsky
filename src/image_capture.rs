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

        let callback = move |image: &Image| {
            println!("Camera thread callback called!");
            ring_buffer.lock().unwrap().add(image.clone());
        };

        match cam.start(callback) {
            Ok(()) => {
                while running.load(Ordering::SeqCst) {
                    {
                        println!("Camera thread is working...");
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
            }
            Err(err) => {
                println!("Failed to start camera: {:?}", err);
            }
        }

        if let Err(err) = cam.stop() {
            println!("Failed to stop camera: {:?}", err);
        }
        cam.close();
        println!("Camera thread exited cleanly.");
    })
}

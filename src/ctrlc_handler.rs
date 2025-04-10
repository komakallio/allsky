use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub fn set_ctrlc_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
}

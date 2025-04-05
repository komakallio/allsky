use libtoupcam_sys::*;
use std::{thread, time};

extern "C" fn callback(event: std::os::raw::c_uint, ctx: *mut std::os::raw::c_void) {
    let callback_context = unsafe { &mut *(ctx as *mut CallbackContext) };
    let event_string = event_to_string(event);
    println!("Callback event received: {event_string}");
    if event == TOUPCAM_EVENT_IMAGE {
        let mut frame_info: ToupcamFrameInfoV4 = unsafe { std::mem::zeroed() };
        let return_code = unsafe {
            Toupcam_PullImageV4(
                callback_context.cam as HToupCam,
                callback_context.image_buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                0,
                0,
                0,
                &mut frame_info,
            )
        };
        println!("Return code: {return_code}");
        println!("Image metadata: {:?}", frame_info.v3);

        let mut exposure_time: u32 = 0;
        let mut gain: u16 = 0;
        unsafe {
            Toupcam_get_ExpoTime(callback_context.cam, &mut exposure_time);
            Toupcam_get_ExpoAGain(callback_context.cam, &mut gain);
        }
        println!("Exposure time {exposure_time} us, gain {gain}");
    }
    println!();
}

struct CallbackContext {
    cam: HToupCam,
    image_buffer: Vec<u8>,
}

fn main() {
    let cam = unsafe { Toupcam_Open(std::ptr::null_mut()) };
    if cam.is_null() {
        println!("Could not open camera!");
        return;
    }

    println!("Camera opened successfully!");

    let mut width = 0;
    let mut height = 0;

    unsafe { Toupcam_get_Size(cam, &mut width, &mut height) };

    println!("Camera size: {} x {}", width, height);

    let image_buffer_length = TDIBWIDTHBYTES(24 * width) * height;
    let image_buffer = vec![0u8; image_buffer_length as usize];

    let mut callback_context = CallbackContext { cam, image_buffer };

    println!("Image buffer length: {}", image_buffer_length);
    println!("Starting image pull mode for 5 seconds...");

    thread::sleep(time::Duration::from_secs(1));

    unsafe {
        Toupcam_StartPullModeWithCallback(
            cam,
            Some(callback),
            &mut callback_context as *mut CallbackContext as *mut std::os::raw::c_void,
        )
    };

    thread::sleep(time::Duration::from_secs(5));

    unsafe { Toupcam_Close(cam) };
}

fn event_to_string(event_type: u32) -> &'static str {
    match event_type {
        TOUPCAM_EVENT_EXPOSURE => "Exposure time or gain changed",
        TOUPCAM_EVENT_TEMPTINT => "White balance changed (Temp/Tint mode)",
        TOUPCAM_EVENT_IMAGE => "Live image arrived",
        TOUPCAM_EVENT_STILLIMAGE => "Snap (still) frame arrived",
        TOUPCAM_EVENT_WBGAIN => "White balance changed (RGB Gain mode)",
        TOUPCAM_EVENT_TRIGGERFAIL => "Trigger failed",
        TOUPCAM_EVENT_BLACK => "Black balance changed",
        TOUPCAM_EVENT_FFC => "Flat field correction status changed",
        TOUPCAM_EVENT_DFC => "Dark field correction status changed",
        TOUPCAM_EVENT_ROI => "ROI changed",
        TOUPCAM_EVENT_LEVELRANGE => "Level range changed",
        TOUPCAM_EVENT_AUTOEXPO_CONV => "Auto exposure convergence",
        TOUPCAM_EVENT_AUTOEXPO_CONVFAIL => "Auto exposure once mode convergence failed",
        TOUPCAM_EVENT_FPNC => "Fix pattern noise correction status changed",
        TOUPCAM_EVENT_ERROR => "Generic error",
        TOUPCAM_EVENT_DISCONNECTED => "Camera disconnected",
        TOUPCAM_EVENT_NOFRAMETIMEOUT => "No frame timeout error",
        TOUPCAM_EVENT_FOCUSPOS => "Focus position",
        TOUPCAM_EVENT_NOPACKETTIMEOUT => "No packet timeout",
        TOUPCAM_EVENT_EXPO_START => "Hardware event: exposure start",
        TOUPCAM_EVENT_EXPO_STOP => "Hardware event: exposure stop",
        TOUPCAM_EVENT_TRIGGER_ALLOW => "Hardware event: next trigger allow",
        TOUPCAM_EVENT_HEARTBEAT => "Hardware event: heartbeat",
        TOUPCAM_EVENT_TRIGGER_IN => "Hardware event: trigger in",
        TOUPCAM_EVENT_FACTORY => "Restore factory settings",
        _ => "Unknown event",
    }
}

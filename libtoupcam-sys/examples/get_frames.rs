use libtoupcam_sys::*;

extern "C" fn callback(event: std::os::raw::c_uint, ctx: *mut std::os::raw::c_void) {
    let callback_context = unsafe { &mut *(ctx as *mut CallbackContext) };
    println!("Callback event received: {event}");
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
    }
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

    unsafe {
        Toupcam_put_AutoExpoEnable(cam, 0);
        Toupcam_put_ExpoTime(cam, 100_000);
        Toupcam_put_ExpoAGain(cam, 100);
    }

    let mut width = 0;
    let mut height = 0;

    unsafe { Toupcam_get_Size(cam, &mut width, &mut height) };

    let image_buffer_length = TDIBWIDTHBYTES(24 * width) * height;
    let image_buffer = vec![0u8; image_buffer_length as usize];

    let mut callback_context = CallbackContext { cam, image_buffer };

    println!("Press Enter to start streaming and stop streaming");
    wait_for_keypress();

    unsafe {
        Toupcam_StartPullModeWithCallback(
            cam,
            Some(callback),
            &mut callback_context as *mut CallbackContext as *mut std::os::raw::c_void,
        )
    };

    wait_for_keypress();

    unsafe { Toupcam_Close(cam) };
}

fn wait_for_keypress() {
    let _ = ::std::io::stdin().read_line(&mut String::new()).unwrap();
}

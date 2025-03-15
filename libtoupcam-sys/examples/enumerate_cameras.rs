use libtoupcam_sys::{TOUPCAM_MAX, Toupcam_EnumV2, ToupcamDeviceV2};

fn main() {
    let cam_count = unsafe { libtoupcam_sys::Toupcam_EnumV2(std::ptr::null_mut()) };
    println!("Found {} cameras", cam_count);

    let mut cam_array: [ToupcamDeviceV2; TOUPCAM_MAX as usize] =
        [unsafe { std::mem::zeroed() }; TOUPCAM_MAX as usize];
    unsafe { Toupcam_EnumV2(cam_array.as_mut_ptr()) };

    for i in 0..TOUPCAM_MAX as usize {
        if cam_array[i].displayname[0] == 0 {
            break;
        }

        let displayname = String::from_utf16_lossy(&cam_array[i].displayname)
            .trim_matches('\0')
            .to_string();

        let id = String::from_utf16_lossy(&cam_array[i].id)
            .trim_matches('\0')
            .to_string();

        println!("Camera {}: {} - {}", i, id, displayname);
    }
}

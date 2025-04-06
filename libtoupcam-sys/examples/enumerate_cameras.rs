use libtoupcam_sys::{TOUPCAM_MAX, Toupcam_EnumV2, ToupcamDeviceV2};
#[cfg(not(target_family = "windows"))]
use std::os::raw::c_chart;
#[cfg(target_family = "windows")]
use std::os::raw::c_ushort;

fn main() {
    let cam_count = unsafe { libtoupcam_sys::Toupcam_EnumV2(std::ptr::null_mut()) };
    println!("Found {cam_count} cameras");

    let mut cam_array = [unsafe { std::mem::zeroed::<ToupcamDeviceV2>() }; TOUPCAM_MAX as usize];
    unsafe { Toupcam_EnumV2(cam_array.as_mut_ptr()) };

    for i in 0..TOUPCAM_MAX as usize {
        if cam_array[i].displayname[0] == 0 {
            break;
        }

        let displayname = characters_to_string(&cam_array[i].displayname);
        let id = characters_to_string(&cam_array[i].id);

        println!("Camera {i}: {id} - {displayname}");
    }
}

#[cfg(target_family = "windows")]
fn characters_to_string(characters: &[c_ushort]) -> String {
    String::from_utf16_lossy(characters)
        .trim_matches('\0')
        .to_string()
}

#[cfg(not(target_family = "windows"))]
fn characters_to_string(characters: &[c_char]) -> String {
    String::from_utf8_lossy(characters)
        .trim_matches('\0')
        .to_string()
}

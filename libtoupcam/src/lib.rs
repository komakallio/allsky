use libtoupcam_sys::*;
#[cfg(not(target_family = "windows"))]
use std::os::raw::c_chart;
#[cfg(target_family = "windows")]
use std::os::raw::c_ushort;

#[derive(Debug)]
pub struct ToupcamDetails {
    pub id: String,
    pub display_name: String,
}

pub fn enumerate_cameras() -> Vec<ToupcamDetails> {
    let mut cam_array = [unsafe { std::mem::zeroed::<ToupcamDeviceV2>() }; TOUPCAM_MAX as usize];
    unsafe { Toupcam_EnumV2(cam_array.as_mut_ptr()) };

    let mut cameras = Vec::new();
    for i in 0..TOUPCAM_MAX as usize {
        if cam_array[i].displayname[0] == 0 {
            break;
        }
        cameras.push(ToupcamDetails {
            id: characters_to_string(&cam_array[i].id),
            display_name: characters_to_string(&cam_array[i].displayname),
        });
    }
    cameras
}

pub fn open() -> HToupCam {
    unsafe { Toupcam_Open(std::ptr::null_mut()) }
}

pub fn open_by_index(index: usize) -> HToupCam {
    unsafe { Toupcam_OpenByIndex(index as u32) }
}

pub fn close(cam: HToupCam) {
    unsafe { Toupcam_Close(cam) }
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

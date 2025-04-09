use libtoupcam_sys as toup;

#[cfg(not(target_family = "windows"))]
use std::os::raw::c_chart;
#[cfg(target_family = "windows")]
use std::os::raw::c_ushort;

#[derive(Debug)]
pub struct ToupcamDevice {
    pub id: String,
    pub display_name: String,
}

pub fn enumerate_cameras() -> Vec<ToupcamDevice> {
    let mut cam_array =
        [unsafe { std::mem::zeroed::<toup::ToupcamDeviceV2>() }; toup::TOUPCAM_MAX as usize];
    unsafe { toup::Toupcam_EnumV2(cam_array.as_mut_ptr()) };

    let mut cameras = Vec::new();
    for i in 0..toup::TOUPCAM_MAX as usize {
        if cam_array[i].displayname[0] == 0 {
            break;
        }
        cameras.push(ToupcamDevice {
            id: characters_to_string(&cam_array[i].id),
            display_name: characters_to_string(&cam_array[i].displayname),
        });
    }
    cameras
}

pub fn open_first() -> toup::HToupCam {
    unsafe { toup::Toupcam_Open(std::ptr::null_mut()) }
}

pub fn open_by_index(index: usize) -> toup::HToupCam {
    unsafe { toup::Toupcam_OpenByIndex(index as u32) }
}

pub fn close(cam: toup::HToupCam) {
    unsafe { toup::Toupcam_Close(cam) }
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

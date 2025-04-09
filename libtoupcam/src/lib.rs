use libtoupcam_sys as toup;

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

pub fn start_pull_mode<T>(
    cam: toup::HToupCam,
    callback: toup::PTOUPCAM_EVENT_CALLBACK,
    context: &mut T,
) -> toup::HRESULT {
    unsafe {
        toup::Toupcam_StartPullModeWithCallback(
            cam,
            callback,
            context as *mut T as *mut std::os::raw::c_void,
        )
    }
}

pub fn pull_image(
    cam: toup::HToupCam,
    buffer: &mut [u8],
    still: bool,
    bits: usize,
    row_pitch: usize,
) -> toup::HRESULT {
    let mut frame_info = unsafe { std::mem::zeroed::<toup::ToupcamFrameInfoV4>() };
    unsafe {
        toup::Toupcam_PullImageV4(
            cam,
            buffer.as_mut_ptr() as *mut std::os::raw::c_void,
            still as i32,
            bits as i32,
            row_pitch as i32,
            &mut frame_info,
        )
    }
}

pub fn stop(cam: toup::HToupCam) -> toup::HRESULT {
    unsafe { toup::Toupcam_Stop(cam) }
}

pub fn get_exposure_time(cam: toup::HToupCam) -> u32 {
    let mut exposure_time: u32 = 0;
    unsafe { toup::Toupcam_get_ExpoTime(cam, &mut exposure_time) };
    exposure_time
}

pub fn get_gain(cam: toup::HToupCam) -> u16 {
    let mut gain: u16 = 0;
    unsafe { toup::Toupcam_get_ExpoAGain(cam, &mut gain) };
    gain
}

#[cfg(target_family = "windows")]
fn characters_to_string(characters: &[std::os::raw::c_ushort]) -> String {
    String::from_utf16_lossy(characters)
        .trim_matches('\0')
        .to_string()
}

#[cfg(not(target_family = "windows"))]
fn characters_to_string(characters: &[std::os::raw::c_char]) -> String {
    String::from_utf8_lossy(characters)
        .trim_matches('\0')
        .to_string()
}

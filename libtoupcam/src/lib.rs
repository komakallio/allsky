use libtoupcam_sys as toup;

#[derive(Debug)]
pub struct ToupcamDevice {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug)]
pub struct ToupcamHandle {
    handle: toup::HToupCam,
}

#[derive(Debug)]
pub enum EventType {
    Image = toup::TOUPCAM_EVENT_IMAGE as isize,
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

pub fn open_first() -> ToupcamHandle {
    unsafe {
        ToupcamHandle {
            handle: toup::Toupcam_Open(std::ptr::null_mut()),
        }
    }
}

pub fn open_by_index(index: usize) -> ToupcamHandle {
    unsafe {
        ToupcamHandle {
            handle: toup::Toupcam_OpenByIndex(index as u32),
        }
    }
}

pub fn close(cam: &ToupcamHandle) {
    unsafe { toup::Toupcam_Close(cam.handle) }
}

pub fn set_raw_mode(cam: &ToupcamHandle, raw_mode: bool) -> toup::HRESULT {
    unsafe { toup::Toupcam_put_Option(cam.handle, toup::TOUPCAM_OPTION_RAW, raw_mode as i32) }
}

pub fn get_resolutions(cam: &ToupcamHandle) -> Vec<(u32, u32)> {
    let camera_model = unsafe { &*toup::Toupcam_query_Model(cam.handle) };
    camera_model
        .res
        .iter()
        .filter(|res| res.width > 0 && res.height > 0)
        .map(|res| (res.width, res.height))
        .collect()
}

pub fn set_resolution_by_index(cam: &ToupcamHandle, index: usize) {
    unsafe { toup::Toupcam_put_eSize(cam.handle, index as u32) };
}

pub fn set_resolution(cam: &ToupcamHandle, width: u32, height: u32) {
    unsafe { toup::Toupcam_put_Size(cam.handle, width as i32, height as i32) };
}

pub fn get_resolution(cam: &ToupcamHandle) -> Result<(u32, u32), toup::HRESULT> {
    let mut width = 0;
    let mut height = 0;
    let result = unsafe { toup::Toupcam_get_Size(cam.handle, &mut width, &mut height) };
    match result {
        result if result >= 0 => Ok((width as u32, height as u32)),
        _ => Err(result),
    }
}

pub fn start_pull_mode<T>(
    cam: &ToupcamHandle,
    callback: toup::PTOUPCAM_EVENT_CALLBACK,
    context: &mut Box<T>,
) -> toup::HRESULT {
    unsafe {
        toup::Toupcam_StartPullModeWithCallback(
            cam.handle,
            callback,
            context.as_mut() as *mut T as *mut std::os::raw::c_void,
        )
    }
}

pub fn pull_image(
    cam: &ToupcamHandle,
    buffer: &mut Vec<u8>,
    still: bool,
    bits: usize,
    row_pitch: usize,
) -> toup::HRESULT {
    let mut frame_info = unsafe { std::mem::zeroed::<toup::ToupcamFrameInfoV4>() };
    unsafe {
        toup::Toupcam_PullImageV4(
            cam.handle,
            buffer.as_mut_ptr() as *mut std::os::raw::c_void,
            still as i32,
            bits as i32,
            row_pitch as i32,
            &mut frame_info,
        )
    }
}

pub fn stop(cam: &ToupcamHandle) -> toup::HRESULT {
    unsafe { toup::Toupcam_Stop(cam.handle) }
}

pub fn get_exposure_time(cam: &ToupcamHandle) -> u32 {
    let mut exposure_time: u32 = 0;
    unsafe { toup::Toupcam_get_ExpoTime(cam.handle, &mut exposure_time) };
    exposure_time
}

pub fn get_gain(cam: &ToupcamHandle) -> u16 {
    let mut gain: u16 = 0;
    unsafe { toup::Toupcam_get_ExpoAGain(cam.handle, &mut gain) };
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

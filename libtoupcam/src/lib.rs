use libtoupcam_sys as toup;

#[derive(Debug)]
pub struct ToupcamDevice {
    pub id: String,
    pub display_name: String,
    handle: Option<toup::HToupCam>,
}

#[derive(Debug)]
pub enum ToupcamError {
    NoHandle,
    OpenHandle,
    Generic(i32),
}

impl ToupcamDevice {
    #[cfg(target_family = "windows")]
    pub fn open(&mut self) -> Result<(), ToupcamError> {
        if self.handle.is_some() {
            return Err(ToupcamError::OpenHandle);
        }

        // Convert to wide characters for Windows
        let wide_id: Vec<u16> = self.id.encode_utf16().chain(std::iter::once(0)).collect();

        self.handle = Some(unsafe { toup::Toupcam_Open(wide_id.as_ptr()) });
        Ok(())
    }

    #[cfg(not(target_family = "windows"))]
    pub fn open(&mut self) -> Result<(), ToupcamError> {
        if self.handle.is_some() {
            return Err(ToupcamError::OpenHandle);
        }

        // Use CString for non-Windows platforms
        use std::ffi::CString;

        let c_id = CString::new(&self.id).expect("Camera ID is an invalid string");

        self.handle = Some(unsafe { toup::Toupcam_Open(c_id.as_ptr()) });
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        unsafe { toup::Toupcam_Close(handle) };
        self.handle = None;
        Ok(())
    }

    pub fn set_raw_mode(&self, enabled: bool) -> Result<(), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let result =
            unsafe { toup::Toupcam_put_Option(handle, toup::TOUPCAM_OPTION_RAW, enabled as i32) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok(())
    }

    pub fn get_resolutions(&self) -> Result<Vec<(u32, u32)>, ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let camera_model = unsafe { &*toup::Toupcam_query_Model(handle) };
        Ok(camera_model
            .res
            .iter()
            .filter(|res| res.width > 0 && res.height > 0)
            .map(|res| (res.width, res.height))
            .collect())
    }

    pub fn get_resolution(&self) -> Result<(u32, u32), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let result = unsafe { toup::Toupcam_get_Size(handle, &mut width, &mut height) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok((width as u32, height as u32))
    }

    pub fn set_resolution_by_index(&self, index: usize) -> Result<(), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let result = unsafe { toup::Toupcam_put_eSize(handle, index as u32) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok(())
    }

    pub fn set_resolution(&self, width: u32, height: u32) -> Result<(), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let result = unsafe { toup::Toupcam_put_Size(handle, width as i32, height as i32) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<(), ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let result = unsafe { toup::Toupcam_Stop(handle) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok(())
    }

    pub fn get_exposure_time(&self) -> Result<u32, ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let mut exposure_time: u32 = 0;
        let result = unsafe { toup::Toupcam_get_ExpoTime(handle, &mut exposure_time) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok(exposure_time)
    }

    pub fn get_gain(&self) -> Result<u16, ToupcamError> {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        let mut gain: u16 = 0;
        let result = unsafe { toup::Toupcam_get_ExpoAGain(handle, &mut gain) };
        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }
        Ok(gain)
    }

    pub fn start<F>(&self, callback: F) -> Result<(), ToupcamError>
    where
        F: FnMut(Vec<u8>) + Send + 'static,
    {
        let Some(handle) = self.handle else {
            return Err(ToupcamError::NoHandle);
        };

        self.set_raw_mode(true)?;

        let (width, height) = self.get_resolution()?;
        // 16-bit raw images are 2 bytes per pixel
        let image_buffer = vec![0u8; (width * height * 2) as usize];

        let callback_context = ToupCallbackContext {
            handle,
            nested_callback: Box::new(callback),
            image_buffer,
        };

        let result = unsafe {
            toup::Toupcam_StartPullModeWithCallback(
                handle,
                Some(toup_event_callback),
                Box::into_raw(Box::new(callback_context)) as *mut std::os::raw::c_void,
            )
        };

        if result < 0 {
            return Err(ToupcamError::Generic(result));
        }

        Ok(())
    }
}

struct ToupCallbackContext {
    handle: toup::HToupCam,
    nested_callback: Box<dyn FnMut(Vec<u8>) + Send + 'static>,
    image_buffer: Vec<u8>,
}

extern "C" fn toup_event_callback(event: std::os::raw::c_uint, context: *mut std::os::raw::c_void) {
    let callback_context = unsafe { &mut *(context as *mut ToupCallbackContext) };
    if event == toup::TOUPCAM_EVENT_IMAGE {
        let mut frame_info = unsafe { std::mem::zeroed::<toup::ToupcamFrameInfoV4>() };
        let result = unsafe {
            toup::Toupcam_PullImageV4(
                callback_context.handle,
                callback_context.image_buffer.as_mut_ptr() as *mut _,
                0,
                0,
                0,
                &mut frame_info,
            )
        };

        if result < 0 {
            println!("Error pulling image: {}", result);
        }

        (callback_context.nested_callback)(callback_context.image_buffer.clone());
    }
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
            handle: None,
        });
    }
    cameras
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

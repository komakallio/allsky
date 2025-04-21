use libtoupcam::EventType;

use crate::image::Image;

use super::{Camera, CameraError};

#[derive(Debug)]
pub(crate) struct ToupcamDevice {
    name: String,
    id: String,
    index: usize,
    handle: Option<libtoupcam::ToupcamHandle>,
}

struct CallbackContext {
    callback: Box<dyn FnMut(&Image) + Send + 'static>,
    image_buffer: Vec<u8>,
    cam_handle: Box<libtoupcam::ToupcamHandle>,
}

extern "C" fn base_callback(event: std::os::raw::c_uint, context: *mut std::os::raw::c_void) {
    println!("Base callback called!");
    let callback_context = unsafe { &mut *(context as *mut CallbackContext) };
    if event == EventType::Image as u32 {
        // TODO: Call pull_image
    }
}

impl Camera for ToupcamDevice {
    fn enumerate_devices() -> Vec<impl Camera> {
        libtoupcam::enumerate_cameras()
            .into_iter()
            .zip(0..)
            .map(|(device, index)| ToupcamDevice {
                name: device.display_name,
                id: device.id,
                index,
                handle: None,
            })
            .collect()
    }

    fn open(&mut self) {
        self.handle = Some(libtoupcam::open_by_index(self.index));
    }

    fn close(&mut self) {
        if let Some(handle) = self.handle.take() {
            libtoupcam::close(&handle);
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn start<F>(&mut self, callback: F) -> Result<(), CameraError>
    where
        F: FnMut(&Image) + Send + 'static,
    {
        if self.handle.is_none() {
            return Err(CameraError::StartError(
                "Camera handle is not initialized".to_string(),
            ));
        }

        match libtoupcam::set_raw_mode(&self.handle.as_ref().unwrap(), true) {
            result if result >= 0 => (),
            _ => {
                return Err(CameraError::StartError(
                    "Failed to set camera to raw mode".to_string(),
                ));
            }
        }

        let Ok((width, height)) = libtoupcam::get_resolution(&self.handle.as_ref().unwrap()) else {
            return Err(CameraError::StartError(
                "Failed to get camera resolution".to_string(),
            ));
        };

        let mut callback_context = CallbackContext {
            callback: Box::new(callback),
            image_buffer: vec![0; (width * height * 2) as usize], // 2 bytes per pixel (16-bit raw image)
            cam_handle: Box::new(self.handle.unwrap()),
        };

        let result = libtoupcam::start_pull_mode(
            self.handle.as_ref().unwrap(),
            Some(base_callback),
            &mut callback_context,
        );

        match result {
            result if result >= 0 => Ok(()),
            _ => Err(CameraError::StartError(format!(
                "Failed to start capturing, result code: {}",
                result
            ))),
        }
    }

    fn stop(&mut self) -> Result<(), CameraError> {
        if let Some(handle) = self.handle.take() {
            let result = libtoupcam::stop(&handle);
            match result {
                result if result >= 0 => Ok(()),
                _ => Err(CameraError::StopError(format!(
                    "Failed to stop camera, result code: {}",
                    result
                ))),
            }
        } else {
            Err(CameraError::StopError(
                "Failed to stop camera because of missing camera handle".to_string(),
            ))
        }
    }
}

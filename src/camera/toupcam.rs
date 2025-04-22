use super::{Camera, CameraError};
use libtoupcam::{self, ToupcamError};

// Implement conversion from ToupcamError to CameraError
impl From<ToupcamError> for CameraError {
    fn from(error: ToupcamError) -> Self {
        match error {
            _ => CameraError::Unknown(format!("Toupcam error: {:?}", error)),
        }
    }
}

impl Camera for libtoupcam::ToupcamDevice {
    fn enumerate_devices() -> Vec<impl Camera> {
        libtoupcam::enumerate_cameras()
    }

    fn get_name(&self) -> String {
        self.display_name.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn open(&mut self) -> Result<(), CameraError> {
        Ok(Self::open(self)?)
    }

    fn close(&mut self) -> Result<(), CameraError> {
        Ok(Self::close(self)?)
    }

    fn start<F>(&mut self, callback: F) -> Result<(), CameraError>
    where
        F: FnMut(Vec<u8>) + Send + 'static,
    {
        Ok(Self::start(&self, callback)?)
    }

    fn stop(&mut self) -> Result<(), CameraError> {
        Ok(Self::stop(self)?)
    }
}

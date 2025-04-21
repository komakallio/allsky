pub(crate) mod toupcam;

#[derive(Debug)]
pub(crate) enum CameraError {
    Unknown(String),
}

pub(crate) trait Camera {
    fn enumerate_devices() -> Vec<impl Camera>;
    fn get_name(&self) -> String;
    fn get_id(&self) -> String;
    fn open(&mut self) -> Result<(), CameraError>;
    fn close(&mut self) -> Result<(), CameraError>;
    fn start<F>(&mut self, callback: F) -> Result<(), CameraError>
    where
        F: FnMut(Vec<u8>) + Send + 'static;
    fn stop(&mut self) -> Result<(), CameraError>;
}

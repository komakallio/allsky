use crate::image::Image;

pub(crate) mod toupcam;

pub(crate) trait Camera {
    fn enumerate_devices() -> Vec<impl Camera>;
    fn get_name(&self) -> String;
    fn get_id(&self) -> String;
    fn open(&mut self);
    fn close(&mut self);
    fn start<F>(&mut self, callback: F)
    where
        F: FnMut(&Image) + Send;
    fn stop(&mut self);
}

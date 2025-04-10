use super::Camera;

pub(crate) struct ToupcamDevice {}
impl Camera for ToupcamDevice {
    fn open(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl ToupcamDevice {
    pub(crate) fn new() -> impl Camera {
        Self {}
    }
}

use super::Camera;

#[derive(Debug)]
pub(crate) struct ToupcamDevice {
    name: String,
    id: String,
    index: usize,
    handle: Option<libtoupcam::ToupcamHandle>,
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
            libtoupcam::close(handle);
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ToupcamDevice {}

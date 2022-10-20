use libasi_sys;

pub fn get_num_of_connected_cameras() -> i32 {
    unsafe { libasi_sys::ASIGetNumOfConnectedCameras() }
}

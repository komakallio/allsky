fn main() {
    println!(
        "Found {} ZWO cameras",
        libasi::get_num_of_connected_cameras()
    );

    let camera_info = libasi::get_camera_property(0);
    println!(
        "Camera ID: {}, Max Width: {}, Max Height: {}",
        camera_info.camera_id, camera_info.max_width, camera_info.max_height
    );
}

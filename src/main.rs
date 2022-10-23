fn main() {
    println!(
        "Found {} ZWO cameras",
        libasi::get_num_of_connected_cameras()
    );

    let camera_info = libasi::get_camera_property(0);
    println!("{:?}", camera_info);

    println!(
        "Opening camera {} with ID {}",
        camera_info.name, camera_info.camera_id
    );
    libasi::open_camera(camera_info.camera_id);

    println!(
        "Closing camera {} with ID {}",
        camera_info.name, camera_info.camera_id
    );
    libasi::close_camera(camera_info.camera_id);
}

fn main() {
    println!(
        "Found {} ZWO cameras",
        libasi::get_num_of_connected_cameras()
    );

    let camera_info = libasi::get_camera_property(0);
    println!("{:?}", camera_info);
}

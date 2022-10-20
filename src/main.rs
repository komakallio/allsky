fn main() {
    println!(
        "Found {} ZWO cameras",
        libasi::get_num_of_connected_cameras()
    );
}

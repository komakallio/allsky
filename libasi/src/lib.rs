use libasi_sys;

pub struct CameraInfo {
    camera_id: i32,
    max_height: i64,
    max_width: i64,
}

pub fn get_num_of_connected_cameras() -> i32 {
    unsafe { libasi_sys::ASIGetNumOfConnectedCameras() }
}

pub fn get_camera_property() -> CameraInfo {
    let mut camera_info = libasi_sys::ASI_CAMERA_INFO {
        Name: [0; 64],
        CameraID: 0,
        MaxHeight: 0,
        MaxWidth: 0,
        IsColorCam: 0,
        BayerPattern: 0,
        SupportedBins: [0; 16],
        SupportedVideoFormat: [0; 8],
        PixelSize: 0.0,
        MechanicalShutter: 0,
        ST4Port: 0,
        IsCoolerCam: 0,
        IsUSB3Host: 0,
        IsUSB3Camera: 0,
        ElecPerADU: 0.0,
        BitDepth: 0,
        IsTriggerCam: 0,
        Unused: [0; 16],
    };
    unsafe {
        let return_value: i32 = libasi_sys::ASIGetCameraProperty(&mut camera_info, 0);
        println!("{}", return_value);
    }

    println!(
        "{} - {}x{}",
        camera_info.CameraID, camera_info.MaxHeight, camera_info.MaxWidth
    );

    CameraInfo {
        camera_id: camera_info.CameraID,
        max_height: camera_info.MaxHeight,
        max_width: camera_info.MaxWidth,
    }
}

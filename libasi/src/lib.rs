use libasi_sys;

#[derive(Debug)]
pub enum BayerPattern {
    RG,
    BG,
    GR,
    GB,
}

#[derive(Debug)]
pub struct CameraInfo {
    pub name: String,
    pub camera_id: i32,
    pub max_height: i64,
    pub max_width: i64,
    pub is_color_cam: bool,
    pub bayer_pattern: BayerPattern,
    pub bit_depth: i32,
    pub is_trigger_cam: bool,
}

pub fn get_num_of_connected_cameras() -> i32 {
    unsafe { libasi_sys::ASIGetNumOfConnectedCameras() }
}

pub fn get_camera_property(camera_idx: i32) -> CameraInfo {
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
    let return_code = unsafe { libasi_sys::ASIGetCameraProperty(&mut camera_info, camera_idx) };

    if return_code != 0 {
        panic!(
            "Could not get camera properties, return code: {}",
            return_code
        );
    }

    // Strip away zero bytes from fixed-length name
    let camera_name_vector = camera_info
        .Name
        .iter()
        .cloned()
        .filter(|&x| x != 0u8)
        .collect::<Vec<_>>();

    CameraInfo {
        name: String::from_utf8(camera_name_vector).unwrap_or(String::from("Invalid name")),
        camera_id: camera_info.CameraID,
        max_height: camera_info.MaxHeight,
        max_width: camera_info.MaxWidth,
        is_color_cam: match camera_info.IsColorCam {
            libasi_sys::ASI_BOOL_ASI_FALSE => false,
            libasi_sys::ASI_BOOL_ASI_TRUE => true,
            _ => panic!("Unrecognized boolean value"),
        },
        bayer_pattern: match camera_info.BayerPattern {
            libasi_sys::ASI_BAYER_PATTERN_ASI_BAYER_RG => BayerPattern::RG,
            libasi_sys::ASI_BAYER_PATTERN_ASI_BAYER_BG => BayerPattern::BG,
            libasi_sys::ASI_BAYER_PATTERN_ASI_BAYER_GR => BayerPattern::GR,
            libasi_sys::ASI_BAYER_PATTERN_ASI_BAYER_GB => BayerPattern::GB,
            _ => panic!("Unrecognized bayer pattern"),
        },
        bit_depth: camera_info.BitDepth,
        is_trigger_cam: match camera_info.IsTriggerCam {
            libasi_sys::ASI_BOOL_ASI_FALSE => false,
            libasi_sys::ASI_BOOL_ASI_TRUE => true,
            _ => panic!("Unrecognized boolean value"),
        },
    }
}

pub fn open_camera(camera_id: i32) {
    let open_return_code = unsafe { libasi_sys::ASIOpenCamera(camera_id) };

    if open_return_code != 0 {
        panic!("Could not open camera, return code: {}", open_return_code);
    }

    let init_return_code = unsafe { libasi_sys::ASIInitCamera(camera_id) };

    if init_return_code != 0 {
        panic!(
            "Could not initialize camera, return code: {}",
            init_return_code
        );
    }
}

pub fn close_camera(camera_id: i32) {
    let return_code = unsafe { libasi_sys::ASICloseCamera(camera_id) };

    if return_code != 0 {
        panic!("Could not close camera, return code: {}", return_code);
    }
}

#[derive(Debug)]
pub struct CameraControl {
    pub name: String,
    pub control_index: i32,
    pub value: i64,
    pub can_auto_adjust: bool,
    pub is_auto_adjusted: bool,
    pub is_writable: bool,
}

pub fn get_controls(camera_id: i32) -> Vec<CameraControl> {
    let mut num_controls: i32 = 0;
    unsafe {
        libasi_sys::ASIGetNumOfControls(camera_id, &mut num_controls);
    }

    let mut controls = Vec::<CameraControl>::with_capacity(num_controls as usize);
    for i in 0..num_controls {
        let mut capability = libasi_sys::ASI_CONTROL_CAPS {
            Name: [0; 64],
            Description: [0; 128],
            MaxValue: 0,
            MinValue: 0,
            DefaultValue: 0,
            IsAutoSupported: 0,
            IsWritable: 0,
            ControlType: 0,
            Unused: [0; 32],
        };
        unsafe {
            libasi_sys::ASIGetControlCaps(camera_id, i, &mut capability);
        }

        let mut value: i64 = 0;
        let mut auto: i32 = 0;
        unsafe {
            libasi_sys::ASIGetControlValue(
                camera_id,
                capability.ControlType as i32,
                &mut value,
                &mut auto,
            );
        }

        let cap_name_vector = capability
            .Name
            .iter()
            .cloned()
            .filter(|&x| x != 0u8)
            .collect::<Vec<_>>();

        controls.push(CameraControl {
            control_index: i,
            name: String::from_utf8(cap_name_vector).unwrap_or(String::from("Invalid name")),
            value: value,
            can_auto_adjust: capability.IsAutoSupported > 0,
            is_auto_adjusted: auto > 0,
            is_writable: capability.IsWritable > 0,
        });
    }

    controls
}

macro_rules! CaptureInfo {
    () => {
        struct CaptureInfo { ln : LiveNode , var_hid : HirId , }
    };
}

CaptureInfo!()
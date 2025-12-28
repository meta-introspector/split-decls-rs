macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < & HSTRING > for std :: ffi :: OsString { fn from (hstring : & HSTRING) -> Self { hstring . to_os_string () } }
    };
}

impl_66!()
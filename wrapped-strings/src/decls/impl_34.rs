macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < & std :: ffi :: OsString > for HSTRING { fn from (value : & std :: ffi :: OsString) -> Self { value . as_os_str () . into () } }
    };
}

impl_34!()
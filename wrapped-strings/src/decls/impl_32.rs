macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < & std :: ffi :: OsStr > for HSTRING { fn from (value : & std :: ffi :: OsStr) -> Self { unsafe { Self :: from_wide_iter (std :: os :: windows :: ffi :: OsStrExt :: encode_wide (value) , value . len () ,) } } }
    };
}

impl_32!()
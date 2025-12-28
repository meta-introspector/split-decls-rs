macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < HSTRING > for std :: ffi :: OsString { fn from (hstring : HSTRING) -> Self { Self :: from (& hstring) } }
    };
}

impl_67!();
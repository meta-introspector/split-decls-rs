macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < HSTRING > for std :: ffi :: OsString { fn eq (& self , other : & HSTRING) -> bool { * other == * * self } }
    };
}

impl_61!()
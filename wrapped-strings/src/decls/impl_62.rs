macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < HSTRING > for & std :: ffi :: OsString { fn eq (& self , other : & HSTRING) -> bool { * other == * * * self } }
    };
}

impl_62!();
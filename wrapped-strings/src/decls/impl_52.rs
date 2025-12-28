macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < std :: ffi :: OsString > for HSTRING { fn eq (& self , other : & std :: ffi :: OsString) -> bool { * self == * * other } }
    };
}

impl_52!()
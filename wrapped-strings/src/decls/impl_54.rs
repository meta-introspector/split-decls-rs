macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < & std :: ffi :: OsString > for HSTRING { fn eq (& self , other : & & std :: ffi :: OsString) -> bool { * self == * * * other } }
    };
}

impl_54!()
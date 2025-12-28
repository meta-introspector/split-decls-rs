macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < std :: ffi :: OsStr > for & HSTRING { fn eq (& self , other : & std :: ffi :: OsStr) -> bool { * * self == * other } }
    };
}

impl_56!()
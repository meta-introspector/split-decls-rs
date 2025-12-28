macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < std :: ffi :: OsStr > for HSTRING { fn eq (& self , other : & std :: ffi :: OsStr) -> bool { self . iter () . copied () . eq (std :: os :: windows :: ffi :: OsStrExt :: encode_wide (other)) } }
    };
}

impl_55!();
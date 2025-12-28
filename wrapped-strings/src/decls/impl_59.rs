macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl PartialEq < HSTRING > for & std :: ffi :: OsStr { fn eq (& self , other : & HSTRING) -> bool { * other == * * self } }
    };
}

impl_59!()
macro_rules! deps {
    () => {
        SmallCStr!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl Deref for SmallCStr { type Target = ffi :: CStr ; # [inline] fn deref (& self) -> & ffi :: CStr { self . as_c_str () } }
    };
}

impl_406!()
macro_rules! deps {
    () => {
        SmallCStr!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl From < & ffi :: CStr > for SmallCStr { fn from (s : & ffi :: CStr) -> Self { Self { data : SmallVec :: from_slice (s . to_bytes_with_nul ()) } } }
    };
}

impl_408!();
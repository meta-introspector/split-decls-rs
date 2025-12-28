macro_rules! deps {
    () => {
        IntoByteSlice!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        unsafe impl < 'a > IntoByteSlice < 'a > for & 'a [u8] { # [inline (always)] fn into_byte_slice (self) -> & 'a [u8] { self } }
    };
}

impl_105!();
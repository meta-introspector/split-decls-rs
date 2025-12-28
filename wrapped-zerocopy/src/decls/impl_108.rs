macro_rules! deps {
    () => {
        IntoByteSlice!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        unsafe impl < 'a > IntoByteSlice < 'a > for & 'a mut [u8] { # [inline (always)] fn into_byte_slice (self) -> & 'a [u8] { self } }
    };
}

impl_108!()
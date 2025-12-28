macro_rules! deps {
    () => {
        IntoByteSliceMut!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        unsafe impl < 'a > IntoByteSliceMut < 'a > for & 'a mut [u8] { # [inline (always)] fn into_byte_slice_mut (self) -> & 'a mut [u8] { self } }
    };
}

impl_109!()
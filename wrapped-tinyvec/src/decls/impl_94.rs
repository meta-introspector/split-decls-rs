macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 's , T > AsMut < [T] > for SliceVec < 's , T > { # [inline (always)] fn as_mut (& mut self) -> & mut [T] { & mut * self } }
    };
}

impl_94!()
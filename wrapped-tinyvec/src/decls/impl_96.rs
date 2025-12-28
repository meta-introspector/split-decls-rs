macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 's , T > Borrow < [T] > for SliceVec < 's , T > { # [inline (always)] fn borrow (& self) -> & [T] { & * self } }
    };
}

impl_96!();
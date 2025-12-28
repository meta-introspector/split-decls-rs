macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 's , T > AsRef < [T] > for SliceVec < 's , T > { # [inline (always)] fn as_ref (& self) -> & [T] { & * self } }
    };
}

impl_95!()
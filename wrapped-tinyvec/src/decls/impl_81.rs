macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 's , T > Default for SliceVec < 's , T > { # [inline (always)] fn default () -> Self { Self { data : & mut [] , len : 0 } } }
    };
}

impl_81!();
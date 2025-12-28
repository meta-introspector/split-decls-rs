macro_rules! deps {
    () => {
        Unalign!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl < T : Copy > Clone for Unalign < T > { # [inline (always)] fn clone (& self) -> Unalign < T > { * self } }
    };
}

impl_410!()
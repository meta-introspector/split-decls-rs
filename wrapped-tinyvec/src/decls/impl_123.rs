macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < A : Array > DerefMut for TinyVec < A > { impl_mirrored ! { type Mirror = TinyVec ; # [inline (always)] # [must_use] fn deref_mut (self : & mut Self) -> & mut Self :: Target ; } }
    };
}

impl_123!()
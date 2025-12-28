macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < A : Array > Deref for TinyVec < A > { type Target = [A :: Item] ; impl_mirrored ! { type Mirror = TinyVec ; # [inline (always)] # [must_use] fn deref (self : & Self) -> & Self :: Target ; } }
    };
}

impl_122!();
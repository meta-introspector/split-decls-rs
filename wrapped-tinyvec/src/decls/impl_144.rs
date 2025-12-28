macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < A : Array > AsRef < [A :: Item] > for TinyVec < A > { # [inline (always)] fn as_ref (& self) -> & [A :: Item] { & * self } }
    };
}

impl_144!()
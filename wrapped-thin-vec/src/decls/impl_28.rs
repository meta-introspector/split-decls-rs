macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > AsRef < [T] > for ThinVec < T > { fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_28!();
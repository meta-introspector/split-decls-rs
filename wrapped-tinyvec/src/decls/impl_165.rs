macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < A : Array > Eq for TinyVec < A > where A :: Item : Eq { }
    };
}

impl_165!();
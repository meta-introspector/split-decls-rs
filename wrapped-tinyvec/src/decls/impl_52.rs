macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < A : Array > Eq for ArrayVec < A > where A :: Item : Eq { }
    };
}

impl_52!();
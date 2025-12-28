macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < A : Array > Eq for ArrayVec < A > where A :: Item : Eq { }
    };
}

impl_52!()
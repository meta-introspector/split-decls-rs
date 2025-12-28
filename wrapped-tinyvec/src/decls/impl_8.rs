macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < A > Copy for ArrayVec < A > where A : Array + Copy , A :: Item : Copy , { }
    };
}

impl_8!();
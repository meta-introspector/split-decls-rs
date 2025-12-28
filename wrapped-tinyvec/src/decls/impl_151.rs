macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < T , A > From < & '_ mut [T] > for TinyVec < A > where T : Clone + Default , A : Array < Item = T > , { # [inline] fn from (slice : & mut [T]) -> Self { Self :: from (& * slice) } }
    };
}

impl_151!()
macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < A , T > FromIterator < A > for InlineArray < A , T > { fn from_iter < I > (it : I) -> Self where I : IntoIterator < Item = A > , { let mut chunk = Self :: new () ; for item in it { chunk . push (item) ; } chunk } }
    };
}

impl_30!();
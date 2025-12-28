macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < A , const N : usize > FromIterator < A > for Chunk < A , N > { fn from_iter < I > (it : I) -> Self where I : IntoIterator < Item = A > , { let mut chunk = Self :: new () ; for item in it { chunk . push_back (item) ; } chunk } }
    };
}

impl_75!()
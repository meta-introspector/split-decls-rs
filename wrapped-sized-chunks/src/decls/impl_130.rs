macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < const N : usize > From < usize > for RawIndex < N > { # [inline] # [must_use] fn from (index : usize) -> Self { debug_assert ! (index < N) ; RawIndex (index) } }
    };
}

impl_130!()
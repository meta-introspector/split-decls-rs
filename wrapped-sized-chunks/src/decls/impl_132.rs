macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < const N : usize > Eq for RawIndex < N > { }
    };
}

impl_132!();
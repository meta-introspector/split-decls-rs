macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < const N : usize > Copy for RawIndex < N > { }
    };
}

impl_128!();
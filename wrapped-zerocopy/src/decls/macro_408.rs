macro_rules! deps {
    () => {
        Unalign!();
    };
}

macro_rules! macro_408 {
    () => {
        deps!();
        impl_known_layout ! (T => Unalign < T >) ;
    };
}

macro_408!();
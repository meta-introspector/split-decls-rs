macro_rules! deps {
    () => {
        MaybeUninit!();
    };
}

macro_rules! macro_368 {
    () => {
        deps!();
        impl_transitive_transmute_from ! (T => MaybeUninit < T > => T => Wrapping < T >) ;
    };
}

macro_368!();
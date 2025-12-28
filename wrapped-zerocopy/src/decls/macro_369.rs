macro_rules! deps {
    () => {
        MaybeUninit!();
    };
}

macro_rules! macro_369 {
    () => {
        deps!();
        impl_transitive_transmute_from ! (T => Wrapping < T > => T => MaybeUninit < T >) ;
    };
}

macro_369!()
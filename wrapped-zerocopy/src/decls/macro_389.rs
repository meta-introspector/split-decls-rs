macro_rules! macro_389 {
    () => {
        impl_transitive_transmute_from ! (T : ? Sized => UnsafeCell < T > => T => Cell < T >) ;
    };
}

macro_389!();
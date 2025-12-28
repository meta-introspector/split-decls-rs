macro_rules! macro_388 {
    () => {
        impl_transitive_transmute_from ! (T : ? Sized => Cell < T > => T => UnsafeCell < T >) ;
    };
}

macro_388!();
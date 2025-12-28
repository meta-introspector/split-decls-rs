macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! macro_271 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + IntoBytes => IntoBytes for UnsafeCell < T > [< T >]) ;
    };
}

macro_271!()
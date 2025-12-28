macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! macro_249 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : IntoBytes => IntoBytes for Wrapping < T > [< T >]) ;
    };
}

macro_249!();
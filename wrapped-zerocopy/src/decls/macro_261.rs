macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! macro_261 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + IntoBytes => IntoBytes for ManuallyDrop < T > [< T >]) ;
    };
}

macro_261!();
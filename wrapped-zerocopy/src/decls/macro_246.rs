macro_rules! deps {
    () => {
        TryFromBytes!();
    };
}

macro_rules! macro_246 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : TryFromBytes => TryFromBytes for Wrapping < T > [< T >]) ;
    };
}

macro_246!()
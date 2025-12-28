macro_rules! deps {
    () => {
        FromBytes!();
    };
}

macro_rules! macro_248 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : FromBytes => FromBytes for Wrapping < T > [< T >]) ;
    };
}

macro_248!()
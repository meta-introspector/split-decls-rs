macro_rules! deps {
    () => {
        FromZeros!();
    };
}

macro_rules! macro_247 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : FromZeros => FromZeros for Wrapping < T > [< T >]) ;
    };
}

macro_247!();
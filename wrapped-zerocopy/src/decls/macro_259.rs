macro_rules! deps {
    () => {
        FromZeros!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromZeros => FromZeros for ManuallyDrop < T > [< T >]) ;
    };
}

macro_259!();
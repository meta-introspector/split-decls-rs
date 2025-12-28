macro_rules! deps {
    () => {
        FromZeros!();
    };
}

macro_rules! macro_265 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromZeros => FromZeros for Cell < T > [UnsafeCell < T >]) ;
    };
}

macro_265!()
macro_rules! deps {
    () => {
        FromZeros!();
    };
}

macro_rules! macro_269 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromZeros => FromZeros for UnsafeCell < T > [< T >]) ;
    };
}

macro_269!()
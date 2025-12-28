macro_rules! deps {
    () => {
        FromBytes!();
    };
}

macro_rules! macro_270 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromBytes => FromBytes for UnsafeCell < T > [< T >]) ;
    };
}

macro_270!()
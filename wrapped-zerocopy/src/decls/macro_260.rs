macro_rules! deps {
    () => {
        FromBytes!();
    };
}

macro_rules! macro_260 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromBytes => FromBytes for ManuallyDrop < T > [< T >]) ;
    };
}

macro_260!();
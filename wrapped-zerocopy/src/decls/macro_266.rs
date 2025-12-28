macro_rules! deps {
    () => {
        FromBytes!();
    };
}

macro_rules! macro_266 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + FromBytes => FromBytes for Cell < T > [UnsafeCell < T >]) ;
    };
}

macro_266!();
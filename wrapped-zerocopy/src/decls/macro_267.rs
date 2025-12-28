macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! macro_267 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + IntoBytes => IntoBytes for Cell < T > [UnsafeCell < T >]) ;
    };
}

macro_267!();
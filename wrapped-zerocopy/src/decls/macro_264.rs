macro_rules! deps {
    () => {
        TryFromBytes!();
    };
}

macro_rules! macro_264 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + TryFromBytes => TryFromBytes for Cell < T > [UnsafeCell < T >]) ;
    };
}

macro_264!();
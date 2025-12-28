macro_rules! deps {
    () => {
        TryFromBytes!();
    };
}

macro_rules! macro_258 {
    () => {
        deps!();
        impl_for_transmute_from ! (T : ? Sized + TryFromBytes => TryFromBytes for ManuallyDrop < T > [< T >]) ;
    };
}

macro_258!()
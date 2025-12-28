macro_rules! deps {
    () => {
        NInt!();
        PInt!();
    };
}

macro_rules! macro_105 {
    () => {
        deps!();
        impl_int_rem ! (NInt , PInt , NInt) ;
    };
}

macro_105!();
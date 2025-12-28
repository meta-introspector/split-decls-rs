macro_rules! deps {
    () => {
        PInt!();
        NInt!();
    };
}

macro_rules! macro_88 {
    () => {
        deps!();
        impl_int_div ! (PInt , NInt , NInt) ;
    };
}

macro_88!()
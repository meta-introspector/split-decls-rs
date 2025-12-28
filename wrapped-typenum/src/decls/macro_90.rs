macro_rules! deps {
    () => {
        PInt!();
        NInt!();
    };
}

macro_rules! macro_90 {
    () => {
        deps!();
        impl_int_div ! (NInt , NInt , PInt) ;
    };
}

macro_90!()
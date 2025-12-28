macro_rules! deps {
    () => {
        PInt!();
        NInt!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        impl_int_div ! (NInt , PInt , NInt) ;
    };
}

macro_89!()
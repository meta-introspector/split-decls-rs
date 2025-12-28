macro_rules! deps {
    () => {
        PInt!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        impl_int_div ! (PInt , PInt , PInt) ;
    };
}

macro_87!();
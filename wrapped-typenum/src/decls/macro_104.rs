macro_rules! deps {
    () => {
        NInt!();
        PInt!();
    };
}

macro_rules! macro_104 {
    () => {
        deps!();
        impl_int_rem ! (PInt , NInt , PInt) ;
    };
}

macro_104!();
macro_rules! deps {
    () => {
        NInt!();
    };
}

macro_rules! macro_106 {
    () => {
        deps!();
        impl_int_rem ! (NInt , NInt , NInt) ;
    };
}

macro_106!()
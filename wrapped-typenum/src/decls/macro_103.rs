macro_rules! deps {
    () => {
        PInt!();
    };
}

macro_rules! macro_103 {
    () => {
        deps!();
        impl_int_rem ! (PInt , PInt , PInt) ;
    };
}

macro_103!()
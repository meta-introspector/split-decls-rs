macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! macro_33 {
    () => {
        deps!();
        impl_low_level_token ! ("group token" proc_macro2 :: Group any_group) ;
    };
}

macro_33!();
macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_34 {
    () => {
        deps!();
        impl_low_level_token ! ("lifetime" Lifetime lifetime) ;
    };
}

macro_34!();
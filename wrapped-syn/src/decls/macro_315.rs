macro_rules! deps {
    () => {
        Turbofish!();
    };
}

macro_rules! macro_315 {
    () => {
        deps!();
        # [cfg (feature = "printing")] generics_wrapper_impls ! (Turbofish) ;
    };
}

macro_315!()
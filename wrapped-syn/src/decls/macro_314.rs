macro_rules! deps {
    () => {
        TypeGenerics!();
    };
}

macro_rules! macro_314 {
    () => {
        deps!();
        # [cfg (feature = "printing")] generics_wrapper_impls ! (TypeGenerics) ;
    };
}

macro_314!();
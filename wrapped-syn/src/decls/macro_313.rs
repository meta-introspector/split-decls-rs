macro_rules! deps {
    () => {
        ImplGenerics!();
    };
}

macro_rules! macro_313 {
    () => {
        deps!();
        # [cfg (feature = "printing")] generics_wrapper_impls ! (ImplGenerics) ;
    };
}

macro_313!()
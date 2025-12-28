macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! parallel_setup {
    () => {
        deps!();
        fn parallel_setup (input : proc_macro2 :: TokenStream , config : Config , prefix : & str ,) -> proc_macro2 :: TokenStream { core_setup (input , & config , prefix , "parallel") }
    };
}

parallel_setup!()
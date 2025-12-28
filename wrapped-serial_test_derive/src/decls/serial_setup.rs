macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! serial_setup {
    () => {
        deps!();
        fn serial_setup (input : proc_macro2 :: TokenStream , config : Config , prefix : & str ,) -> proc_macro2 :: TokenStream { core_setup (input , & config , prefix , "serial") }
    };
}

serial_setup!();
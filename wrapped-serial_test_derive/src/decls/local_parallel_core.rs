macro_rules! local_parallel_core {
    () => {
        fn local_parallel_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; parallel_setup (input , config , "local") }
    };
}

local_parallel_core!();
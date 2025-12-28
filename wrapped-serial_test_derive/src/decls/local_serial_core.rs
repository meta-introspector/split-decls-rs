macro_rules! local_serial_core {
    () => {
        fn local_serial_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; serial_setup (input , config , "local") }
    };
}

local_serial_core!();
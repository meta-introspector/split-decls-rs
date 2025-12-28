macro_rules! deps {
    () => {
        PassMode!();
    };
}

macro_rules! gen_call_with_arg {
    () => {
        deps!();
        fn gen_call_with_arg (func_name : & TokenStream , arg : & TokenStream , pass_mode : PassMode ,) -> TokenStream { match pass_mode { PassMode :: AsIs => quote ! (# func_name (# arg)) , PassMode :: InsertRef => quote ! (# func_name (&# arg)) , PassMode :: Packed => { quote ! (({ let __typesize_internal_temp = # arg ; # func_name (& __typesize_internal_temp) })) } } }
    };
}

gen_call_with_arg!();
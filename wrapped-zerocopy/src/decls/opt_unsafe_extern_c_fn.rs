macro_rules! opt_unsafe_extern_c_fn {
    () => {
        # [doc = " Expands to an `Option<unsafe extern \"C\" fn>` type with the given argument"] # [doc = " types and return type. Designed for use with `unsafe_impl_for_power_set`."] macro_rules ! opt_unsafe_extern_c_fn { ($ ($ args : ident) ,* -> $ ret : ident) => { Option < unsafe extern "C" fn ($ ($ args) ,*) -> $ ret > } ; }
    };
}

opt_unsafe_extern_c_fn!()
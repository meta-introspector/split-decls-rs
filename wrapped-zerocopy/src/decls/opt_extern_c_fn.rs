macro_rules! opt_extern_c_fn {
    () => {
        # [doc = " Expands to an `Option<extern \"C\" fn>` type with the given argument types and"] # [doc = " return type. Designed for use with `unsafe_impl_for_power_set`."] macro_rules ! opt_extern_c_fn { ($ ($ args : ident) ,* -> $ ret : ident) => { Option < extern "C" fn ($ ($ args) ,*) -> $ ret > } ; }
    };
}

opt_extern_c_fn!()
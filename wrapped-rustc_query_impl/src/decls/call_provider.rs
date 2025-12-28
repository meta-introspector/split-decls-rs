macro_rules! call_provider {
    () => {
        macro_rules ! call_provider { ([] [$ tcx : expr , $ name : ident , $ key : expr]) => { { ($ tcx . query_system . fns . local_providers .$ name) ($ tcx , $ key) } } ; ([(separate_provide_extern) $ ($ rest : tt) *] [$ tcx : expr , $ name : ident , $ key : expr]) => { { if let Some (key) = $ key . as_local_key () { ($ tcx . query_system . fns . local_providers .$ name) ($ tcx , key) } else { ($ tcx . query_system . fns . extern_providers .$ name) ($ tcx , $ key) } } } ; ([$ other : tt $ ($ modifiers : tt) *] [$ ($ args : tt) *]) => { call_provider ! ([$ ($ modifiers) *] [$ ($ args) *]) } ; }
    };
}

call_provider!();
macro_rules! warn_event {
    () => {
        macro_rules ! warn_event { ($ ($ x : tt) *) => { crate :: tracing :: event ! (WARN , $ ($ x) *) } ; }
    };
}

warn_event!();
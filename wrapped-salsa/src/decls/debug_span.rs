macro_rules! debug_span {
    () => {
        macro_rules ! debug_span { ($ ($ x : tt) *) => { crate :: tracing :: span ! (DEBUG , $ ($ x) *) } ; }
    };
}

debug_span!();
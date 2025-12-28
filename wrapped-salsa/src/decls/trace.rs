macro_rules! trace {
    () => {
        macro_rules ! trace { ($ ($ x : tt) *) => { crate :: tracing :: event ! (TRACE , $ ($ x) *) } ; }
    };
}

trace!();
macro_rules! debug {
    () => {
        macro_rules ! debug { ($ ($ x : tt) *) => { crate :: tracing :: event ! (DEBUG , $ ($ x) *) } ; }
    };
}

debug!()
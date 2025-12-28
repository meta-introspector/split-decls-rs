macro_rules! info {
    () => {
        macro_rules ! info { ($ ($ x : tt) *) => { crate :: tracing :: event ! (INFO , $ ($ x) *) } ; }
    };
}

info!();
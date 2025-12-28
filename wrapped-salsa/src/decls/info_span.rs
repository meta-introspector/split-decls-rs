macro_rules! info_span {
    () => {
        # [expect (unused_macros)] macro_rules ! info_span { ($ ($ x : tt) *) => { crate :: tracing :: span ! (INFO , $ ($ x) *) } ; }
    };
}

info_span!();
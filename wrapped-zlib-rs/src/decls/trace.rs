macro_rules! trace {
    () => {
        # [macro_export] macro_rules ! trace { ($ ($ arg : tt) *) => { # [cfg (feature = "ZLIB_DEBUG")] { eprint ! ($ ($ arg) *) } } ; }
    };
}

trace!()
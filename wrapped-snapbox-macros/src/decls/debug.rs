macro_rules! debug {
    () => {
        # [doc = " Feature-flag controlled additional test debug information"] # [cfg (not (feature = "debug"))] # [macro_export] macro_rules ! debug { ($ ($ arg : tt) *) => { } ; }
    };
}

debug!();
macro_rules! register_jar {
    () => {
        # [cfg (not (feature = "inventory"))] # [macro_export] # [doc (hidden)] macro_rules ! register_jar { ($ ($ _ : tt) *) => { } ; }
    };
}

register_jar!();
macro_rules! impl_clone_for_custom_punctuation {
    () => {
        # [cfg (not (feature = "clone-impls"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_clone_for_custom_punctuation { ($ ident : ident , $ ($ tt : tt) +) => { } ; }
    };
}

impl_clone_for_custom_punctuation!();
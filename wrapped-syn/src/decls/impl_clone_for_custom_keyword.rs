macro_rules! impl_clone_for_custom_keyword {
    () => {
        # [cfg (not (feature = "clone-impls"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_clone_for_custom_keyword { ($ ident : ident) => { } ; }
    };
}

impl_clone_for_custom_keyword!()
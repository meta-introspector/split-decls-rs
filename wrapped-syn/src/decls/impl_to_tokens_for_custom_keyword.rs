macro_rules! impl_to_tokens_for_custom_keyword {
    () => {
        # [cfg (not (feature = "printing"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_to_tokens_for_custom_keyword { ($ ident : ident) => { } ; }
    };
}

impl_to_tokens_for_custom_keyword!()
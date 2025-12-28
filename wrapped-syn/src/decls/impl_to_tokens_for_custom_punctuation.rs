macro_rules! impl_to_tokens_for_custom_punctuation {
    () => {
        # [cfg (not (feature = "printing"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_to_tokens_for_custom_punctuation { ($ ident : ident , $ ($ tt : tt) +) => { } ; }
    };
}

impl_to_tokens_for_custom_punctuation!()
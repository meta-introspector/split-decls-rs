macro_rules! impl_parse_for_custom_punctuation {
    () => {
        # [cfg (not (feature = "parsing"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_parse_for_custom_punctuation { ($ ident : ident , $ ($ tt : tt) +) => { } ; }
    };
}

impl_parse_for_custom_punctuation!()
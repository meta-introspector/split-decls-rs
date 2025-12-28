macro_rules! impl_parse_for_custom_keyword {
    () => {
        # [cfg (not (feature = "parsing"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_parse_for_custom_keyword { ($ ident : ident) => { } ; }
    };
}

impl_parse_for_custom_keyword!()
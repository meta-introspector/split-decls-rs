macro_rules! impl_extra_traits_for_custom_punctuation {
    () => {
        # [cfg (not (feature = "extra-traits"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_extra_traits_for_custom_punctuation { ($ ident : ident , $ ($ tt : tt) +) => { } ; }
    };
}

impl_extra_traits_for_custom_punctuation!();
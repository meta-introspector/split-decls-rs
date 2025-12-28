macro_rules! impl_extra_traits_for_custom_keyword {
    () => {
        # [cfg (not (feature = "extra-traits"))] # [doc (hidden)] # [macro_export] macro_rules ! impl_extra_traits_for_custom_keyword { ($ ident : ident) => { } ; }
    };
}

impl_extra_traits_for_custom_keyword!()
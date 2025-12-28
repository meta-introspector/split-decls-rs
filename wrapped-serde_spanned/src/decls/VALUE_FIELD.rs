macro_rules! VALUE_FIELD {
    () => {
        # [cfg (feature = "serde")] pub (crate) const VALUE_FIELD : & str = "$__serde_spanned_private_value" ;
    };
}

VALUE_FIELD!();
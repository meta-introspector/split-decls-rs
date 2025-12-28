macro_rules! START_FIELD {
    () => {
        # [cfg (feature = "serde")] pub (crate) const START_FIELD : & str = "$__serde_spanned_private_start" ;
    };
}

START_FIELD!();
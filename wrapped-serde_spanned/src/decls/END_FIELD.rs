macro_rules! END_FIELD {
    () => {
        # [cfg (feature = "serde")] pub (crate) const END_FIELD : & str = "$__serde_spanned_private_end" ;
    };
}

END_FIELD!()
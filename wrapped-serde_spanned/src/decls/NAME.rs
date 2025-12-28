macro_rules! NAME {
    () => {
        # [cfg (feature = "serde")] pub (crate) const NAME : & str = "$__serde_spanned_private_Spanned" ;
    };
}

NAME!()
macro_rules! FIELD {
    () => {
        # [cfg (feature = "serde")] pub (crate) const FIELD : & str = "$__toml_private_datetime" ;
    };
}

FIELD!();
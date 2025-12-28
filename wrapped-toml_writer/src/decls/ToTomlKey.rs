macro_rules! ToTomlKey {
    () => {
        # [cfg (feature = "alloc")] pub trait ToTomlKey { fn to_toml_key (& self) -> String ; }
    };
}

ToTomlKey!();
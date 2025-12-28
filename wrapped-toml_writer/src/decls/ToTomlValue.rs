macro_rules! ToTomlValue {
    () => {
        # [cfg (feature = "alloc")] pub trait ToTomlValue { fn to_toml_value (& self) -> String ; }
    };
}

ToTomlValue!()
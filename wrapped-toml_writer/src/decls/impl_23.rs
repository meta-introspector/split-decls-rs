macro_rules! deps {
    () => {
        ToTomlKey!();
        WriteTomlKey!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < T > ToTomlKey for T where T : WriteTomlKey + ? Sized , { fn to_toml_key (& self) -> String { let mut result = String :: new () ; let _ = self . write_toml_key (& mut result) ; result } }
    };
}

impl_23!();
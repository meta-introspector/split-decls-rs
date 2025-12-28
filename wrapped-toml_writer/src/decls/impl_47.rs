macro_rules! deps {
    () => {
        ToTomlValue!();
        WriteTomlValue!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < T > ToTomlValue for T where T : WriteTomlValue + ? Sized , { fn to_toml_value (& self) -> String { let mut result = String :: new () ; let _ = self . write_toml_value (& mut result) ; result } }
    };
}

impl_47!();
macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl WriteTomlValue for Cow < '_ , str > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_ref () . write_toml_value (writer) } }
    };
}

impl_65!();
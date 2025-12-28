macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl WriteTomlValue for String { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_str () . write_toml_value (writer) } }
    };
}

impl_64!()
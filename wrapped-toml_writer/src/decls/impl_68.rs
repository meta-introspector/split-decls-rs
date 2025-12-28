macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < V : WriteTomlValue > WriteTomlValue for Vec < V > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_slice () . write_toml_value (writer) } }
    };
}

impl_68!();
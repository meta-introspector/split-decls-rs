macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
        TomlInteger!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl crate :: WriteTomlValue for TomlInteger < u128 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
    };
}

impl_14!()
macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
        TomlInteger!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl crate :: WriteTomlValue for TomlInteger < u16 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
    };
}

impl_8!();
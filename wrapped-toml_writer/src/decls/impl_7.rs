macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlInteger!();
        TomlWrite!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl crate :: WriteTomlValue for TomlInteger < i8 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
    };
}

impl_7!()
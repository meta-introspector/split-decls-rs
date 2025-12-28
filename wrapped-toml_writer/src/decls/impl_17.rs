macro_rules! deps {
    () => {
        TomlInteger!();
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl crate :: WriteTomlValue for TomlInteger < isize > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
    };
}

impl_17!()
macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl WriteTomlValue for i16 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
    };
}

impl_53!();
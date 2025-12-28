macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl WriteTomlValue for i32 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
    };
}

impl_55!()
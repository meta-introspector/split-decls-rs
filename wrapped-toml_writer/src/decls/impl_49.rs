macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl WriteTomlValue for bool { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
    };
}

impl_49!()
macro_rules! deps {
    () => {
        TomlWrite!();
        TomlStringBuilder!();
        WriteTomlValue!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl WriteTomlValue for str { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { crate :: TomlStringBuilder :: new (self) . as_default () . write_toml_value (writer) } }
    };
}

impl_63!()
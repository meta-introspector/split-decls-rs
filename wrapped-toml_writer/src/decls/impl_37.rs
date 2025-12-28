macro_rules! deps {
    () => {
        WriteTomlKey!();
        TomlWrite!();
        TomlKey!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl crate :: WriteTomlKey for TomlKey < '_ > { fn write_toml_key < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { let newline = false ; write_toml_value (self . decoded , self . encoding , newline , writer) } }
    };
}

impl_37!()
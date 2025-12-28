macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
        TomlString!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl crate :: WriteTomlValue for TomlString < '_ > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write_toml_value (self . decoded , Some (self . encoding) , self . newline , writer) } }
    };
}

impl_33!();
macro_rules! deps {
    () => {
        TomlWrite!();
    };
}

macro_rules! WriteTomlValue {
    () => {
        deps!();
        pub trait WriteTomlValue { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result ; }
    };
}

WriteTomlValue!();
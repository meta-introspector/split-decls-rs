macro_rules! deps {
    () => {
        TomlWrite!();
    };
}

macro_rules! WriteTomlKey {
    () => {
        deps!();
        pub trait WriteTomlKey { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result ; }
    };
}

WriteTomlKey!();
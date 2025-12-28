macro_rules! deps {
    () => {
        WriteTomlKey!();
        TomlWrite!();
        TomlKeyBuilder!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl WriteTomlKey for str { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { crate :: TomlKeyBuilder :: new (self) . as_default () . write_toml_key (writer) } }
    };
}

impl_25!()
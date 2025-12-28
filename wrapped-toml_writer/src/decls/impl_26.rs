macro_rules! deps {
    () => {
        WriteTomlKey!();
        TomlWrite!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl WriteTomlKey for String { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_str () . write_toml_key (writer) } }
    };
}

impl_26!()
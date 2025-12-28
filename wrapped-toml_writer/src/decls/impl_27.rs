macro_rules! deps {
    () => {
        WriteTomlKey!();
        TomlWrite!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl WriteTomlKey for Cow < '_ , str > { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_ref () . write_toml_key (writer) } }
    };
}

impl_27!()
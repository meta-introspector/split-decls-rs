macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlKey!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < V : WriteTomlKey + ? Sized > WriteTomlKey for & V { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { (* self) . write_toml_key (writer) } }
    };
}

impl_28!()
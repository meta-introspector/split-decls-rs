macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < V : WriteTomlValue + ? Sized > WriteTomlValue for & V { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { (* self) . write_toml_value (writer) } }
    };
}

impl_72!()
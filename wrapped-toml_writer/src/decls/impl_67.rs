macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < V : WriteTomlValue , const N : usize > WriteTomlValue for [V ; N] { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_slice () . write_toml_value (writer) } }
    };
}

impl_67!();
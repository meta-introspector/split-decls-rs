macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl WriteTomlValue for char { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { let mut buf = [0 ; 4] ; let v = self . encode_utf8 (& mut buf) ; v . write_toml_value (writer) } }
    };
}

impl_62!();
macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlWrite!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl WriteTomlValue for f32 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { match (self . is_sign_negative () , self . is_nan () , * self == 0.0) { (true , true , _) => write ! (writer , "-nan") , (false , true , _) => write ! (writer , "nan") , (true , false , true) => write ! (writer , "-0.0") , (false , false , true) => write ! (writer , "0.0") , (_ , false , false) => { if self % 1.0 == 0.0 { write ! (writer , "{self}.0") } else { write ! (writer , "{self}") } } } } }
    };
}

impl_60!()
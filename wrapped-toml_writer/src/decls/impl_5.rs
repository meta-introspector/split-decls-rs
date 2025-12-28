macro_rules! deps {
    () => {
        WriteTomlValue!();
        TomlIntegerFormat!();
        TomlInteger!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < N > TomlInteger < N > where Self : crate :: WriteTomlValue , { # [doc = " Apply default formatting"] pub fn new (value : N) -> Self { Self { value , format : TomlIntegerFormat :: new () , } } }
    };
}

impl_5!()
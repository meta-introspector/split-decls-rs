macro_rules! deps {
    () => {
        Error!();
        TomlError!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl std :: error :: Error for TomlError { }
    };
}

impl_67!();
macro_rules! deps {
    () => {
        Directory!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl From < OsString > for Directory { fn from (os_string : OsString) -> Self { Directory :: new (PathBuf :: from (os_string)) } }
    };
}

impl_61!()
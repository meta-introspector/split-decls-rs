macro_rules! deps {
    () => {
        DataSource!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl From < std :: path :: PathBuf > for DataSource { fn from (value : std :: path :: PathBuf) -> Self { Self :: path (value) } }
    };
}

impl_89!()
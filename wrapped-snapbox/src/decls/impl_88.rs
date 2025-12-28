macro_rules! deps {
    () => {
        DataSource!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl From < & '_ std :: path :: Path > for DataSource { fn from (value : & '_ std :: path :: Path) -> Self { Self :: path (value) } }
    };
}

impl_88!()
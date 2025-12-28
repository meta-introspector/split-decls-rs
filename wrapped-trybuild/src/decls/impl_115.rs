macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl AsRef < OsStr > for Name { fn as_ref (& self) -> & OsStr { self . 0 . as_ref () } }
    };
}

impl_115!();
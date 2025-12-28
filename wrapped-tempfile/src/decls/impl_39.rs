macro_rules! deps {
    () => {
        TempPath!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl AsRef < OsStr > for TempPath { fn as_ref (& self) -> & OsStr { self . path . as_os_str () } }
    };
}

impl_39!()
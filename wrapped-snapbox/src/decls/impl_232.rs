macro_rules! deps {
    () => {
        RedactedValue!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl From < & 'static Path > for RedactedValue { fn from (inner : & 'static Path) -> Self { inner . to_owned () . into () } }
    };
}

impl_232!()
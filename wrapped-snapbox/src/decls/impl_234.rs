macro_rules! deps {
    () => {
        RedactedValue!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl From < & '_ PathBuf > for RedactedValue { fn from (inner : & '_ PathBuf) -> Self { inner . clone () . into () } }
    };
}

impl_234!();
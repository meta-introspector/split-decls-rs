macro_rules! deps {
    () => {
        RedactedValue!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl From < & '_ String > for RedactedValue { fn from (inner : & '_ String) -> Self { inner . clone () . into () } }
    };
}

impl_230!();
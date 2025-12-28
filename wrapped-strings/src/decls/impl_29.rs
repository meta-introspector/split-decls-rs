macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < String > for HSTRING { fn from (value : String) -> Self { value . as_str () . into () } }
    };
}

impl_29!();
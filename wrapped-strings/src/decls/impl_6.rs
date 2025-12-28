macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl From < String > for BSTR { fn from (value : String) -> Self { value . as_str () . into () } }
    };
}

impl_6!();
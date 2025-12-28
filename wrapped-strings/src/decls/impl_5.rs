macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < & str > for BSTR { fn from (value : & str) -> Self { let value : alloc :: vec :: Vec < u16 > = value . encode_utf16 () . collect () ; Self :: from_wide (& value) } }
    };
}

impl_5!()
macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'b > From < & 'b String > for Value { fn from (s : & 'b String) -> Self { s . to_owned () . into () } }
    };
}

impl_258!();
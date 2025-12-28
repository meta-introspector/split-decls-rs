macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'b > From < & 'b str > for Value { fn from (s : & 'b str) -> Self { s . to_owned () . into () } }
    };
}

impl_257!()
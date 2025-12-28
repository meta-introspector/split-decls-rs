macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'b > From < & 'b Self > for Value { fn from (s : & 'b Self) -> Self { s . clone () } }
    };
}

impl_256!()
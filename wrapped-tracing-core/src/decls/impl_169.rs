macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl AsRef < str > for Field { fn as_ref (& self) -> & str { self . name () } }
    };
}

impl_169!()
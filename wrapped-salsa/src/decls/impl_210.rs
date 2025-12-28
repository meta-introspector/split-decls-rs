macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl Lookup < String > for & str { fn into_owned (self) -> String { self . to_owned () } }
    };
}

impl_210!()
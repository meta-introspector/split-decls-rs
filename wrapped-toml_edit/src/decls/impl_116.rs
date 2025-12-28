macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'b > From < & 'b Self > for Item { fn from (s : & 'b Self) -> Self { s . clone () } }
    };
}

impl_116!();
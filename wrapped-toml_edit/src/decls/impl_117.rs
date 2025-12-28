macro_rules! deps {
    () => {
        Table!();
        Item!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl From < Table > for Item { fn from (s : Table) -> Self { Self :: Table (s) } }
    };
}

impl_117!()
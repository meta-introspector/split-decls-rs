macro_rules! deps {
    () => {
        ArrayOfTables!();
        Item!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl From < ArrayOfTables > for Item { fn from (s : ArrayOfTables) -> Self { Self :: ArrayOfTables (s) } }
    };
}

impl_118!()
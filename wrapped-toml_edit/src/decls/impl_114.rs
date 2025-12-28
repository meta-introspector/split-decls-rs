macro_rules! deps {
    () => {
        ArrayOfTables!();
        Table!();
        Item!();
        Value!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Clone for Item { # [inline (never)] fn clone (& self) -> Self { match self { Self :: None => Self :: None , Self :: Value (v) => Self :: Value (v . clone ()) , Self :: Table (v) => Self :: Table (v . clone ()) , Self :: ArrayOfTables (v) => Self :: ArrayOfTables (v . clone ()) , } } }
    };
}

impl_114!();
macro_rules! deps {
    () => {
        InlineTable!();
        Value!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl From < InlineTable > for Value { fn from (table : InlineTable) -> Self { Self :: InlineTable (table) } }
    };
}

impl_267!()
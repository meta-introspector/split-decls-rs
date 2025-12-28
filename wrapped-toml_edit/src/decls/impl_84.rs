macro_rules! deps {
    () => {
        InlineTable!();
        Index!();
        Value!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 's > ops :: Index < & 's str > for InlineTable { type Output = Value ; fn index (& self , key : & 's str) -> & Value { self . get (key) . expect ("index not found") } }
    };
}

impl_84!();
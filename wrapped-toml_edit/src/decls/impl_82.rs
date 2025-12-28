macro_rules! deps {
    () => {
        Table!();
        Item!();
        Index!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 's > ops :: Index < & 's str > for Table { type Output = Item ; fn index (& self , key : & 's str) -> & Item { self . get (key) . expect ("index not found") } }
    };
}

impl_82!();
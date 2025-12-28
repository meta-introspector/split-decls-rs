macro_rules! deps {
    () => {
        Index!();
        Item!();
        DocumentMut!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 's > ops :: Index < & 's str > for DocumentMut { type Output = Item ; fn index (& self , key : & 's str) -> & Item { self . root . index (key) } }
    };
}

impl_86!();
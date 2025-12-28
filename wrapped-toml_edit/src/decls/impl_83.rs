macro_rules! deps {
    () => {
        Item!();
        Table!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < 's > ops :: IndexMut < & 's str > for Table { fn index_mut (& mut self , key : & 's str) -> & mut Item { self . entry (key) . or_insert (Item :: None) } }
    };
}

impl_83!();
macro_rules! deps {
    () => {
        InlineTable!();
        Value!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < 's > ops :: IndexMut < & 's str > for InlineTable { fn index_mut (& mut self , key : & 's str) -> & mut Value { self . get_mut (key) . expect ("index not found") } }
    };
}

impl_85!()
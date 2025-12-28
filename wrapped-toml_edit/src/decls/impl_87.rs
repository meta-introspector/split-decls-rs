macro_rules! deps {
    () => {
        DocumentMut!();
        Item!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 's > ops :: IndexMut < & 's str > for DocumentMut { fn index_mut (& mut self , key : & 's str) -> & mut Item { self . root . index_mut (key) } }
    };
}

impl_87!();
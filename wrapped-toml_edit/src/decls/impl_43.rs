macro_rules! deps {
    () => {
        Table!();
        Item!();
        DocumentMut!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < Table > for DocumentMut { fn from (root : Table) -> Self { Self { root : Item :: Table (root) , .. Default :: default () } } }
    };
}

impl_43!()
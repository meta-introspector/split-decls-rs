macro_rules! deps {
    () => {
        Item!();
        Table!();
        ArrayOfTables!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Extend < Table > for ArrayOfTables { fn extend < T : IntoIterator < Item = Table > > (& mut self , iter : T) { for value in iter { self . push (value) ; } } }
    };
}

impl_21!();
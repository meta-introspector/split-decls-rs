macro_rules! deps {
    () => {
        Table!();
        Item!();
    };
}

macro_rules! ArrayOfTablesIterMut {
    () => {
        deps!();
        # [doc = " An iterator type over [`ArrayOfTables`]'s [`Table`]s"] pub type ArrayOfTablesIterMut < 'a > = Box < dyn Iterator < Item = & 'a mut Table > + 'a > ;
    };
}

ArrayOfTablesIterMut!();
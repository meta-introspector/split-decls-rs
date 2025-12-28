macro_rules! deps {
    () => {
        Item!();
        Table!();
    };
}

macro_rules! ArrayOfTablesIter {
    () => {
        deps!();
        # [doc = " An iterator type over [`ArrayOfTables`]'s [`Table`]s"] pub type ArrayOfTablesIter < 'a > = Box < dyn Iterator < Item = & 'a Table > + 'a > ;
    };
}

ArrayOfTablesIter!()
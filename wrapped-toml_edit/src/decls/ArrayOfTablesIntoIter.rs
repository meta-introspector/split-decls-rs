macro_rules! deps {
    () => {
        Table!();
        Item!();
    };
}

macro_rules! ArrayOfTablesIntoIter {
    () => {
        deps!();
        # [doc = " An iterator type over [`ArrayOfTables`]'s [`Table`]s"] pub type ArrayOfTablesIntoIter = Box < dyn Iterator < Item = Table > > ;
    };
}

ArrayOfTablesIntoIter!()
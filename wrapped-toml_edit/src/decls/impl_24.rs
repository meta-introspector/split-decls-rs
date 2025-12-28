macro_rules! deps {
    () => {
        ArrayOfTables!();
        ArrayOfTablesIter!();
        Item!();
        IntoIter!();
        Table!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 's > IntoIterator for & 's ArrayOfTables { type Item = & 's Table ; type IntoIter = ArrayOfTablesIter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_24!();
macro_rules! deps {
    () => {
        Table!();
        Item!();
        ArrayOfTables!();
        IntoIter!();
        ArrayOfTablesIntoIter!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl IntoIterator for ArrayOfTables { type Item = Table ; type IntoIter = ArrayOfTablesIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . values . into_iter () . filter (| v | v . is_table ()) . map (| v | v . into_table () . unwrap ()) ,) } }
    };
}

impl_23!();
// Generated macro for impl_32 (impl)
macro_rules! Depcrate_array_of_tablesimpl_32 {
() => {
// Module: crate::array_of_tables
// Provides: {"impl_32"}
// Dependencies: {}
impl IntoIterator for ArrayOfTables { type Item = Table ; type IntoIter = ArrayOfTablesIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . values . into_iter () . filter (| v | v . is_table ()) . map (| v | v . into_table () . unwrap ()) ,) } }
};
}

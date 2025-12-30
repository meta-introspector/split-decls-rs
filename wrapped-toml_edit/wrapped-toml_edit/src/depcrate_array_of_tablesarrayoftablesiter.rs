// Generated macro for ArrayOfTablesIter (type)
macro_rules! Depcrate_array_of_tablesArrayOfTablesIter {
() => {
// Module: crate::array_of_tables
// Provides: {"ArrayOfTablesIter"}
// Dependencies: {}
# [doc = " An iterator type over [`ArrayOfTables`]'s [`Table`]s"] pub type ArrayOfTablesIter < 'a > = Box < dyn Iterator < Item = & 'a Table > + 'a > ;
};
}

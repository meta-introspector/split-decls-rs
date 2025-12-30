// Generated macro for ArrayOfTablesIterMut (type)
macro_rules! Depcrate_array_of_tablesArrayOfTablesIterMut {
() => {
// Module: crate::array_of_tables
// Provides: {"ArrayOfTablesIterMut"}
// Dependencies: {}
# [doc = " An iterator type over [`ArrayOfTables`]'s [`Table`]s"] pub type ArrayOfTablesIterMut < 'a > = Box < dyn Iterator < Item = & 'a mut Table > + 'a > ;
};
}

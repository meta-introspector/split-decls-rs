// Generated macro for impl_30 (impl)
macro_rules! Depcrate_array_of_tablesimpl_30 {
() => {
// Module: crate::array_of_tables
// Provides: {"impl_30"}
// Dependencies: {}
impl Extend < Table > for ArrayOfTables { fn extend < T : IntoIterator < Item = Table > > (& mut self , iter : T) { for value in iter { self . push (value) ; } } }
};
}

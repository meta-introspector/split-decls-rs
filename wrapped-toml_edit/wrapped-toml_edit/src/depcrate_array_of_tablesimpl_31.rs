// Generated macro for impl_31 (impl)
macro_rules! Depcrate_array_of_tablesimpl_31 {
() => {
// Module: crate::array_of_tables
// Provides: {"impl_31"}
// Dependencies: {}
impl FromIterator < Table > for ArrayOfTables { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Table > , { let v = iter . into_iter () . map (Item :: Table) ; Self { values : v . collect () , span : None , } } }
};
}

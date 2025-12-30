// Generated macro for visit_array_of_tables_mut (function)
macro_rules! Depcrate_visit_mutvisit_array_of_tables_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_array_of_tables_mut"}
// Dependencies: {}
pub fn visit_array_of_tables_mut < V > (v : & mut V , node : & mut ArrayOfTables) where V : VisitMut + ? Sized , { for table in node . iter_mut () { v . visit_table_mut (table) ; } }
};
}

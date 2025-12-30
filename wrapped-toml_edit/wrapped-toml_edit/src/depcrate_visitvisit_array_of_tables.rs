// Generated macro for visit_array_of_tables (function)
macro_rules! Depcrate_visitvisit_array_of_tables {
() => {
// Module: crate::visit
// Provides: {"visit_array_of_tables"}
// Dependencies: {}
pub fn visit_array_of_tables < 'doc , V > (v : & mut V , node : & 'doc ArrayOfTables) where V : Visit < 'doc > + ? Sized , { for table in node . iter () { v . visit_table (table) ; } }
};
}

// Generated macro for visit_table (function)
macro_rules! Depcrate_visitvisit_table {
() => {
// Module: crate::visit
// Provides: {"visit_table"}
// Dependencies: {}
pub fn visit_table < 'doc , V > (v : & mut V , node : & 'doc Table) where V : Visit < 'doc > + ? Sized , { v . visit_table_like (node) ; }
};
}

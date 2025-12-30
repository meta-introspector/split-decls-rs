// Generated macro for visit_table_mut (function)
macro_rules! Depcrate_visit_mutvisit_table_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_table_mut"}
// Dependencies: {}
pub fn visit_table_mut < V > (v : & mut V , node : & mut Table) where V : VisitMut + ? Sized , { v . visit_table_like_mut (node) ; }
};
}

// Generated macro for visit_document_mut (function)
macro_rules! Depcrate_visit_mutvisit_document_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_document_mut"}
// Dependencies: {}
pub fn visit_document_mut < V > (v : & mut V , node : & mut DocumentMut) where V : VisitMut + ? Sized , { v . visit_table_mut (node . as_table_mut ()) ; }
};
}

// Generated macro for visit_document (function)
macro_rules! Depcrate_visitvisit_document {
() => {
// Module: crate::visit
// Provides: {"visit_document"}
// Dependencies: {}
pub fn visit_document < 'doc , V > (v : & mut V , node : & 'doc DocumentMut) where V : Visit < 'doc > + ? Sized , { v . visit_table (node . as_table ()) ; }
};
}

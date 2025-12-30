// Generated macro for visit_inline_table (function)
macro_rules! Depcrate_visitvisit_inline_table {
() => {
// Module: crate::visit
// Provides: {"visit_inline_table"}
// Dependencies: {}
pub fn visit_inline_table < 'doc , V > (v : & mut V , node : & 'doc InlineTable) where V : Visit < 'doc > + ? Sized , { v . visit_table_like (node) ; }
};
}

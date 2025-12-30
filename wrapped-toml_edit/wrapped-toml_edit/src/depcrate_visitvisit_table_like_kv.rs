// Generated macro for visit_table_like_kv (function)
macro_rules! Depcrate_visitvisit_table_like_kv {
() => {
// Module: crate::visit
// Provides: {"visit_table_like_kv"}
// Dependencies: {}
pub fn visit_table_like_kv < 'doc , V > (v : & mut V , _key : & 'doc str , node : & 'doc Item) where V : Visit < 'doc > + ? Sized , { v . visit_item (node) ; }
};
}

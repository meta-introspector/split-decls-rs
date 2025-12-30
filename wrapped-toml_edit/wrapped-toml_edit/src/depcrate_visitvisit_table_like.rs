// Generated macro for visit_table_like (function)
macro_rules! Depcrate_visitvisit_table_like {
() => {
// Module: crate::visit
// Provides: {"visit_table_like"}
// Dependencies: {}
pub fn visit_table_like < 'doc , V > (v : & mut V , node : & 'doc dyn TableLike) where V : Visit < 'doc > + ? Sized , { for (key , item) in node . iter () { v . visit_table_like_kv (key , item) ; } }
};
}

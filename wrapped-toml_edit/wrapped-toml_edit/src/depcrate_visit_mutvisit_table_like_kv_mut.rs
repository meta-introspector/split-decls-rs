// Generated macro for visit_table_like_kv_mut (function)
macro_rules! Depcrate_visit_mutvisit_table_like_kv_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_table_like_kv_mut"}
// Dependencies: {}
pub fn visit_table_like_kv_mut < V > (v : & mut V , _key : KeyMut < '_ > , node : & mut Item) where V : VisitMut + ? Sized , { v . visit_item_mut (node) ; }
};
}

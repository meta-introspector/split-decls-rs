// Generated macro for visit_table_like_mut (function)
macro_rules! Depcrate_visit_mutvisit_table_like_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_table_like_mut"}
// Dependencies: {}
pub fn visit_table_like_mut < V > (v : & mut V , node : & mut dyn TableLike) where V : VisitMut + ? Sized , { for (key , item) in node . iter_mut () { v . visit_table_like_kv_mut (key , item) ; } }
};
}

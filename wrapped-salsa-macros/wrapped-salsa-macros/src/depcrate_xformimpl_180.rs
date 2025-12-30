// Generated macro for impl_180 (impl)
macro_rules! Depcrate_xformimpl_180 {
() => {
// Module: crate::xform
// Provides: {"impl_180"}
// Dependencies: {}
impl ChangeLt < '_ > { pub fn elided_to (db_lt : & syn :: Lifetime) -> Self { ChangeLt { from : Some ("_") , to : db_lt . ident . to_string () , } } pub fn in_type (mut self , ty : & syn :: Type) -> syn :: Type { let mut ty = ty . clone () ; self . visit_type_mut (& mut ty) ; ty } }
};
}

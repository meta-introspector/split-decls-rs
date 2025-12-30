// Generated macro for impl_61 (impl)
macro_rules! Depcrate_expandimpl_61 {
() => {
// Module: crate::expand
// Provides: {"impl_61"}
// Dependencies: {}
impl VisitMut for IdentAndTypesRenamer < '_ > { # [allow (clippy :: cmp_owned)] fn visit_ident_mut (& mut self , id : & mut Ident) { for (old_ident , new_ident) in & self . idents { if id . to_string () == old_ident . to_string () { * id = new_ident . clone () ; } } } fn visit_type_mut (& mut self , ty : & mut Type) { for (type_name , new_type) in & self . types { if let Type :: Path (TypePath { path , .. }) = ty { if path_to_string (path) == * type_name { * ty = Type :: Path (new_type . clone ()) ; } } } } }
};
}

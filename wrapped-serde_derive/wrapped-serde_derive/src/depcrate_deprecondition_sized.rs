// Generated macro for precondition_sized (function)
macro_rules! Depcrate_deprecondition_sized {
() => {
// Module: crate::de
// Provides: {"precondition_sized"}
// Dependencies: {}
fn precondition_sized (cx : & Ctxt , cont : & Container) { if let Data :: Struct (_ , fields) = & cont . data { if let Some (last) = fields . last () { if let syn :: Type :: Slice (_) = ungroup (last . ty) { cx . error_spanned_by (cont . original , "cannot deserialize a dynamically sized struct" ,) ; } } } }
};
}

// Generated macro for erase_impl_trait (function)
macro_rules! Depcrate_expanderase_impl_trait {
() => {
// Module: crate::expand
// Provides: {"erase_impl_trait"}
// Dependencies: {}
fn erase_impl_trait (ty : & Type) -> Type { let mut ty = ty . clone () ; ImplTraitEraser . visit_type_mut (& mut ty) ; ty }
};
}

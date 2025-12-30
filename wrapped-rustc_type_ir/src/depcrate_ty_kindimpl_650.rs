// Generated macro for impl_650 (impl)
macro_rules! Depcrate_ty_kindimpl_650 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_650"}
// Dependencies: {}
impl FloatVarValue { pub fn is_known (self) -> bool { match self { FloatVarValue :: Known (_) => true , FloatVarValue :: Unknown => false , } } pub fn is_unknown (self) -> bool { ! self . is_known () } }
};
}

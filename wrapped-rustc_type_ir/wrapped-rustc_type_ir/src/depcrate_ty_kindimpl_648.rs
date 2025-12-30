// Generated macro for impl_648 (impl)
macro_rules! Depcrate_ty_kindimpl_648 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_648"}
// Dependencies: {}
impl IntVarValue { pub fn is_known (self) -> bool { match self { IntVarValue :: IntType (_) | IntVarValue :: UintType (_) => true , IntVarValue :: Unknown => false , } } pub fn is_unknown (self) -> bool { ! self . is_known () } }
};
}

// Generated macro for impl_158 (impl)
macro_rules! Depcrate_idl_typeimpl_158 {
() => {
// Module: crate::idl_type
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for FloatType { fn to_idl_type (& self , _record : & FirstPassRecord < 'a >) -> IdlType < 'a > { if self . unrestricted . is_some () { IdlType :: UnrestrictedFloat } else { IdlType :: Float } } }
};
}

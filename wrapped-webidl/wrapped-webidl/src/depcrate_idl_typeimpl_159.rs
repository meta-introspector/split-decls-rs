// Generated macro for impl_159 (impl)
macro_rules! Depcrate_idl_typeimpl_159 {
() => {
// Module: crate::idl_type
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for DoubleType { fn to_idl_type (& self , _record : & FirstPassRecord < 'a >) -> IdlType < 'a > { if self . unrestricted . is_some () { IdlType :: UnrestrictedDouble } else { IdlType :: Double } } }
};
}

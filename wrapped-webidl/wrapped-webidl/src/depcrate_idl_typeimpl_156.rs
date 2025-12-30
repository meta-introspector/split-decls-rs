// Generated macro for impl_156 (impl)
macro_rules! Depcrate_idl_typeimpl_156 {
() => {
// Module: crate::idl_type
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for ShortType { fn to_idl_type (& self , _record : & FirstPassRecord < 'a >) -> IdlType < 'a > { if self . unsigned . is_some () { IdlType :: UnsignedShort } else { IdlType :: Short } } }
};
}

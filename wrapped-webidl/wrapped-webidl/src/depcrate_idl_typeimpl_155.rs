// Generated macro for impl_155 (impl)
macro_rules! Depcrate_idl_typeimpl_155 {
() => {
// Module: crate::idl_type
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for LongType { fn to_idl_type (& self , _record : & FirstPassRecord < 'a >) -> IdlType < 'a > { if self . unsigned . is_some () { IdlType :: UnsignedLong } else { IdlType :: Long } } }
};
}

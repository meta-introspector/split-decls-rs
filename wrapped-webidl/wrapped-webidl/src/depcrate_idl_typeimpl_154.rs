// Generated macro for impl_154 (impl)
macro_rules! Depcrate_idl_typeimpl_154 {
() => {
// Module: crate::idl_type
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for LongLongType { fn to_idl_type (& self , _record : & FirstPassRecord < 'a >) -> IdlType < 'a > { if self . unsigned . is_some () { IdlType :: UnsignedLongLong } else { IdlType :: LongLong } } }
};
}

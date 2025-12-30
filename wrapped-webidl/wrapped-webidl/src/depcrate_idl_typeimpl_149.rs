// Generated macro for impl_149 (impl)
macro_rules! Depcrate_idl_typeimpl_149 {
() => {
// Module: crate::idl_type
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for FrozenArrayType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { IdlType :: FrozenArray (Box :: new (self . generics . body . to_idl_type (record))) } }
};
}

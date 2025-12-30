// Generated macro for impl_152 (impl)
macro_rules! Depcrate_idl_typeimpl_152 {
() => {
// Module: crate::idl_type
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for PromiseType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { IdlType :: Promise (Box :: new (self . generics . body . to_idl_type (record))) } }
};
}

// Generated macro for impl_148 (impl)
macro_rules! Depcrate_idl_typeimpl_148 {
() => {
// Module: crate::idl_type
// Provides: {"impl_148"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for SequenceType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { IdlType :: Sequence (Box :: new (self . generics . body . to_idl_type (record))) } }
};
}

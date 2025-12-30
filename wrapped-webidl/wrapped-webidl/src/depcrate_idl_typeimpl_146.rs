// Generated macro for impl_146 (impl)
macro_rules! Depcrate_idl_typeimpl_146 {
() => {
// Module: crate::idl_type
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for SingleType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { SingleType :: Any (t) => t . to_idl_type (record) , SingleType :: NonAny (t) => t . to_idl_type (record) , } } }
};
}

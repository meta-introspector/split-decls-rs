// Generated macro for impl_164 (impl)
macro_rules! Depcrate_idl_typeimpl_164 {
() => {
// Module: crate::idl_type
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for ReturnType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { ReturnType :: Undefined (t) => t . to_idl_type (record) , ReturnType :: Type (t) => t . to_idl_type (record) , } } }
};
}

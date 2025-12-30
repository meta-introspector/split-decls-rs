// Generated macro for impl_145 (impl)
macro_rules! Depcrate_idl_typeimpl_145 {
() => {
// Module: crate::idl_type
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for Type < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { Type :: Single (t) => t . to_idl_type (record) , Type :: Union (t) => t . to_idl_type (record) , } } }
};
}

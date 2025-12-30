// Generated macro for impl_162 (impl)
macro_rules! Depcrate_idl_typeimpl_162 {
() => {
// Module: crate::idl_type
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for UnionMemberType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { UnionMemberType :: Single (t) => t . to_idl_type (record) , UnionMemberType :: Union (t) => t . to_idl_type (record) , } } }
};
}

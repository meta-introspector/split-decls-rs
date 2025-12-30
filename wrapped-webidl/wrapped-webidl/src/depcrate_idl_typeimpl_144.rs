// Generated macro for impl_144 (impl)
macro_rules! Depcrate_idl_typeimpl_144 {
() => {
// Module: crate::idl_type
// Provides: {"impl_144"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for UnionType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { let mut idl_types = Vec :: with_capacity (self . body . list . len ()) ; for t in & self . body . list { idl_types . push (t . to_idl_type (record)) ; } IdlType :: Union (idl_types) } }
};
}

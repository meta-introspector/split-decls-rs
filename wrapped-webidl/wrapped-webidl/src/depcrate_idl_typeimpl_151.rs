// Generated macro for impl_151 (impl)
macro_rules! Depcrate_idl_typeimpl_151 {
() => {
// Module: crate::idl_type
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a , T : ToIdlType < 'a > > ToIdlType < 'a > for MayBeNull < T > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { let inner_idl_type = self . type_ . to_idl_type (record) ; if self . q_mark . is_some () { IdlType :: Nullable (Box :: new (inner_idl_type)) } else { inner_idl_type } } }
};
}

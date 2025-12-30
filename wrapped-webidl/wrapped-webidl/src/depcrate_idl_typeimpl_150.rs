// Generated macro for impl_150 (impl)
macro_rules! Depcrate_idl_typeimpl_150 {
() => {
// Module: crate::idl_type
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for ObservableArrayType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { IdlType :: ObservableArray (Box :: new (self . generics . body . to_idl_type (record))) } }
};
}

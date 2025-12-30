// Generated macro for impl_160 (impl)
macro_rules! Depcrate_idl_typeimpl_160 {
() => {
// Module: crate::idl_type
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for RecordType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { IdlType :: Record (Box :: new (self . generics . body . 0 . to_idl_type (record)) , Box :: new (self . generics . body . 2 . to_idl_type (record)) ,) } }
};
}

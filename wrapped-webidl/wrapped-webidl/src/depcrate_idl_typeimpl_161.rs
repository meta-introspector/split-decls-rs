// Generated macro for impl_161 (impl)
macro_rules! Depcrate_idl_typeimpl_161 {
() => {
// Module: crate::idl_type
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for RecordKeyType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { RecordKeyType :: Byte (t) => t . to_idl_type (record) , RecordKeyType :: DOM (t) => t . to_idl_type (record) , RecordKeyType :: USV (t) => t . to_idl_type (record) , RecordKeyType :: NonAny (t) => t . to_idl_type (record) , } } }
};
}

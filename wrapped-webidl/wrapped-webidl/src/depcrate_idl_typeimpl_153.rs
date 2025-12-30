// Generated macro for impl_153 (impl)
macro_rules! Depcrate_idl_typeimpl_153 {
() => {
// Module: crate::idl_type
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for IntegerType { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { IntegerType :: LongLong (t) => t . to_idl_type (record) , IntegerType :: Long (t) => t . to_idl_type (record) , IntegerType :: Short (t) => t . to_idl_type (record) , } } }
};
}

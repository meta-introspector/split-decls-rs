// Generated macro for impl_157 (impl)
macro_rules! Depcrate_idl_typeimpl_157 {
() => {
// Module: crate::idl_type
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for FloatingPointType { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { FloatingPointType :: Float (t) => t . to_idl_type (record) , FloatingPointType :: Double (t) => t . to_idl_type (record) , } } }
};
}

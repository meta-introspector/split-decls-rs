// Generated macro for impl_163 (impl)
macro_rules! Depcrate_idl_typeimpl_163 {
() => {
// Module: crate::idl_type
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for ConstType < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { match self { ConstType :: Integer (t) => t . to_idl_type (record) , ConstType :: FloatingPoint (t) => t . to_idl_type (record) , ConstType :: Boolean (t) => t . to_idl_type (record) , ConstType :: Byte (t) => t . to_idl_type (record) , ConstType :: Octet (t) => t . to_idl_type (record) , ConstType :: Identifier (t) => t . to_idl_type (record) , } } }
};
}

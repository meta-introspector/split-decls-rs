// Generated macro for impl_137 (impl)
macro_rules! Depcrate_intrinsicimpl_137 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_137"}
// Dependencies: {}
impl StaticDefinition { pub fn as_variable (& self) -> Option < (String , (TypeKind , VariableType)) > { match self { StaticDefinition :: Constant (arg) => Some ((arg . name . to_string () , (arg . kind . clone () , VariableType :: Argument) ,)) , StaticDefinition :: Generic (..) => None , } } }
};
}

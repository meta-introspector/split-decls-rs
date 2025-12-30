// Generated macro for impl_139 (impl)
macro_rules! Depcrate_intrinsicimpl_139 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_139"}
// Dependencies: {}
impl fmt :: Display for StaticDefinition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { StaticDefinition :: Constant (arg) => write ! (f , "const {arg}") , StaticDefinition :: Generic (generic) => write ! (f , "{generic}") , } } }
};
}

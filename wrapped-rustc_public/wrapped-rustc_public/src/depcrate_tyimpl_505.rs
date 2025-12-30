// Generated macro for impl_505 (impl)
macro_rules! Depcrate_tyimpl_505 {
() => {
// Module: crate::ty
// Provides: {"impl_505"}
// Dependencies: {}
impl Display for AdtKind { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { AdtKind :: Enum => "enum" , AdtKind :: Union => "union" , AdtKind :: Struct => "struct" , }) } }
};
}

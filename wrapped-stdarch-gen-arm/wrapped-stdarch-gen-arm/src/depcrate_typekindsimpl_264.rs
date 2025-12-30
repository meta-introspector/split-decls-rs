// Generated macro for impl_264 (impl)
macro_rules! Depcrate_typekindsimpl_264 {
() => {
// Module: crate::typekinds
// Provides: {"impl_264"}
// Dependencies: {}
impl TypeKindOptions { pub fn contains (& self , kind : BaseTypeKind) -> bool { match kind { BaseTypeKind :: Float => self . f , BaseTypeKind :: Int => self . s , BaseTypeKind :: UInt => self . u , BaseTypeKind :: Poly => self . p , BaseTypeKind :: Bool => false , } } }
};
}

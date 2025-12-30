// Generated macro for impl_279 (impl)
macro_rules! Depcrate_typekindsimpl_279 {
() => {
// Module: crate::typekinds
// Provides: {"impl_279"}
// Dependencies: {}
impl From < & TypeKind > for usize { fn from (ty : & TypeKind) -> Self { match ty { TypeKind :: Base (_) => 1 , TypeKind :: Pointer (_ , _) => 2 , TypeKind :: Vector (_) => 3 , TypeKind :: Custom (_) => 4 , TypeKind :: Wildcard (_) => 5 , } } }
};
}

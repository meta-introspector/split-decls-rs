// Generated macro for impl_352 (impl)
macro_rules! Depcrate_binderimpl_352 {
() => {
// Module: crate::binder
// Provides: {"impl_352"}
// Dependencies: {}
impl < 's , I : Interner , Iter : IntoIterator > EarlyBinder < I , Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { pub fn iter_instantiated_copied (self , cx : I , args : & 's [I :: GenericArg] ,) -> IterInstantiatedCopied < 's , I , Iter > { IterInstantiatedCopied { it : self . value . into_iter () , cx , args } } # [doc = " Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),"] # [doc = " but on an iterator of values that deref to a `TypeFoldable`."] pub fn iter_identity_copied (self) -> IterIdentityCopied < Iter > { IterIdentityCopied { it : self . value . into_iter () } } }
};
}

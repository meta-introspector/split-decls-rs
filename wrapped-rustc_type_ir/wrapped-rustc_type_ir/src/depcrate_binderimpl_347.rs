// Generated macro for impl_347 (impl)
macro_rules! Depcrate_binderimpl_347 {
() => {
// Module: crate::binder
// Provides: {"impl_347"}
// Dependencies: {}
impl < I : Interner , Iter : IntoIterator > EarlyBinder < I , Iter > where Iter :: Item : TypeFoldable < I > , { pub fn iter_instantiated < A > (self , cx : I , args : A) -> IterInstantiated < I , Iter , A > where A : SliceLike < Item = I :: GenericArg > , { IterInstantiated { it : self . value . into_iter () , cx , args } } # [doc = " Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),"] # [doc = " but on an iterator of `TypeFoldable` values."] pub fn iter_identity (self) -> Iter :: IntoIter { self . value . into_iter () } }
};
}

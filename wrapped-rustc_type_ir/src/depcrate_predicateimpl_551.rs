// Generated macro for impl_551 (impl)
macro_rules! Depcrate_predicateimpl_551 {
() => {
// Module: crate::predicate
// Provides: {"impl_551"}
// Dependencies: {}
impl AliasTermKind { pub fn descr (self) -> & 'static str { match self { AliasTermKind :: ProjectionTy => "associated type" , AliasTermKind :: ProjectionConst => "associated const" , AliasTermKind :: InherentTy => "inherent associated type" , AliasTermKind :: InherentConst => "inherent associated const" , AliasTermKind :: OpaqueTy => "opaque type" , AliasTermKind :: FreeTy => "type alias" , AliasTermKind :: FreeConst => "unevaluated constant" , AliasTermKind :: UnevaluatedConst => "unevaluated constant" , } } pub fn is_type (self) -> bool { match self { AliasTermKind :: ProjectionTy | AliasTermKind :: InherentTy | AliasTermKind :: OpaqueTy | AliasTermKind :: FreeTy => true , AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst | AliasTermKind :: InherentConst | AliasTermKind :: FreeConst => false , } } }
};
}

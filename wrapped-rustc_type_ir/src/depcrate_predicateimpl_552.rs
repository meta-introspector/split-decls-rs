// Generated macro for impl_552 (impl)
macro_rules! Depcrate_predicateimpl_552 {
() => {
// Module: crate::predicate
// Provides: {"impl_552"}
// Dependencies: {}
impl From < ty :: AliasTyKind > for AliasTermKind { fn from (value : ty :: AliasTyKind) -> Self { match value { ty :: Projection => AliasTermKind :: ProjectionTy , ty :: Opaque => AliasTermKind :: OpaqueTy , ty :: Free => AliasTermKind :: FreeTy , ty :: Inherent => AliasTermKind :: InherentTy , } } }
};
}

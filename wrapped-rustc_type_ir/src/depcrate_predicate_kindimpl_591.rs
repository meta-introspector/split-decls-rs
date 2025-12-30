// Generated macro for impl_591 (impl)
macro_rules! Depcrate_predicate_kindimpl_591 {
() => {
// Module: crate::predicate_kind
// Provides: {"impl_591"}
// Dependencies: {}
impl std :: fmt :: Display for AliasRelationDirection { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { AliasRelationDirection :: Equate => write ! (f , "==") , AliasRelationDirection :: Subtype => write ! (f , "<:") , } } }
};
}

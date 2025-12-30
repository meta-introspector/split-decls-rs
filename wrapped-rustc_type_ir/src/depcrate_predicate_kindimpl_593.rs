// Generated macro for impl_593 (impl)
macro_rules! Depcrate_predicate_kindimpl_593 {
() => {
// Module: crate::predicate_kind
// Provides: {"impl_593"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for PredicateKind < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PredicateKind :: Clause (a) => a . fmt (f) , PredicateKind :: Subtype (pair) => pair . fmt (f) , PredicateKind :: Coerce (pair) => pair . fmt (f) , PredicateKind :: DynCompatible (trait_def_id) => { write ! (f , "DynCompatible({trait_def_id:?})") } PredicateKind :: ConstEquate (c1 , c2) => write ! (f , "ConstEquate({c1:?}, {c2:?})") , PredicateKind :: Ambiguous => write ! (f , "Ambiguous") , PredicateKind :: NormalizesTo (p) => p . fmt (f) , PredicateKind :: AliasRelate (t1 , t2 , dir) => { write ! (f , "AliasRelate({t1:?}, {dir:?}, {t2:?})") } } } }
};
}

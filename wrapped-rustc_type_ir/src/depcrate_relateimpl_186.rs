// Generated macro for impl_186 (impl)
macro_rules! Depcrate_relateimpl_186 {
() => {
// Module: crate::relate
// Provides: {"impl_186"}
// Dependencies: {}
impl < I : Interner > Relate < I > for ty :: TraitPredicate < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: TraitPredicate < I > , b : ty :: TraitPredicate < I > ,) -> RelateResult < I , ty :: TraitPredicate < I > > { let trait_ref = relation . relate (a . trait_ref , b . trait_ref) ? ; if a . polarity != b . polarity { return Err (TypeError :: PolarityMismatch (ExpectedFound :: new (a . polarity , b . polarity))) ; } Ok (ty :: TraitPredicate { trait_ref , polarity : a . polarity }) } }
};
}

// Generated macro for impl_559 (impl)
macro_rules! Depcrate_predicateimpl_559 {
() => {
// Module: crate::predicate
// Provides: {"impl_559"}
// Dependencies: {}
impl < I : Interner > From < ty :: UnevaluatedConst < I > > for AliasTerm < I > { fn from (ct : ty :: UnevaluatedConst < I >) -> Self { AliasTerm { args : ct . args , def_id : ct . def , _use_alias_term_new_instead : () } } }
};
}

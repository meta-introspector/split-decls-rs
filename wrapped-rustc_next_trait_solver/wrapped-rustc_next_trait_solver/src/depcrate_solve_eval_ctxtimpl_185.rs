// Generated macro for impl_185 (impl)
macro_rules! Depcrate_solve_eval_ctxtimpl_185 {
() => {
// Module: crate::solve::eval_ctxt
// Provides: {"impl_185"}
// Dependencies: {}
impl CurrentGoalKind { fn from_query_input < I : Interner > (cx : I , input : QueryInput < I , I :: Predicate >) -> CurrentGoalKind { match input . goal . predicate . kind () . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (pred)) => { if cx . trait_is_coinductive (pred . trait_ref . def_id) { CurrentGoalKind :: CoinductiveTrait } else { CurrentGoalKind :: Misc } } ty :: PredicateKind :: NormalizesTo (_) => CurrentGoalKind :: NormalizesTo , _ => CurrentGoalKind :: Misc , } } }
};
}

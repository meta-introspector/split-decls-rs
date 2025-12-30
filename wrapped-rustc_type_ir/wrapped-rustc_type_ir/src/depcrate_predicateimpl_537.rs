// Generated macro for impl_537 (impl)
macro_rules! Depcrate_predicateimpl_537 {
() => {
// Module: crate::predicate
// Provides: {"impl_537"}
// Dependencies: {}
impl PredicatePolarity { # [doc = " Flips polarity by turning `Positive` into `Negative` and `Negative` into `Positive`."] pub fn flip (& self) -> PredicatePolarity { match self { PredicatePolarity :: Positive => PredicatePolarity :: Negative , PredicatePolarity :: Negative => PredicatePolarity :: Positive , } } }
};
}

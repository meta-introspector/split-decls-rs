// Generated macro for PredicatePolarity (enum)
macro_rules! Depcrate_predicatePredicatePolarity {
() => {
// Module: crate::predicate
// Provides: {"PredicatePolarity"}
// Dependencies: {}
# [doc = " Polarity for a trait predicate. May either be negative or positive."] # [doc = " Distinguished from [`ImplPolarity`] since we never compute goals with"] # [doc = " \"reservation\" level."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum PredicatePolarity { # [doc = " `Type: Trait`"] Positive , # [doc = " `Type: !Trait`"] Negative , }
};
}

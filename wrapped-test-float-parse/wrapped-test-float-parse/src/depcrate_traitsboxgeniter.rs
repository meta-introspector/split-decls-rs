// Generated macro for BoxGenIter (type)
macro_rules! Depcrate_traitsBoxGenIter {
() => {
// Module: crate::traits
// Provides: {"BoxGenIter"}
// Dependencies: {}
# [doc = " For tests that use iterator combinators, it is easier to just to box the iterator than trying"] # [doc = " to specify its type. This is a shorthand for the usual type."] pub type BoxGenIter < This , F > = Box < dyn Iterator < Item = < This as Generator < F > > :: WriteCtx > + Send > ;
};
}

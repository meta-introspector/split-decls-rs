// Generated macro for Extrema (struct)
macro_rules! Depcrate_aggregateExtrema {
() => {
// Module: crate::aggregate
// Provides: {"Extrema"}
// Dependencies: {}
pub struct Extrema < T , S = () > { # [doc = " Number of `smallest`/`largest` values to keep track of."] limit : usize , pub smallest : BTreeMap < T , ExtremaSources < S > > , pub largest : BTreeMap < T , ExtremaSources < S > > , }
};
}

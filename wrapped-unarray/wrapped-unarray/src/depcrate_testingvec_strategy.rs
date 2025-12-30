// Generated macro for vec_strategy (function)
macro_rules! Depcrate_testingvec_strategy {
() => {
// Module: crate::testing
// Provides: {"vec_strategy"}
// Dependencies: {}
pub fn vec_strategy (n : usize) -> impl Strategy < Value = Vec < String > > { vec (any :: < String > () , n) }
};
}

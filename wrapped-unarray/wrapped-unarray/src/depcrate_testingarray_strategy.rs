// Generated macro for array_strategy (function)
macro_rules! Depcrate_testingarray_strategy {
() => {
// Module: crate::testing
// Provides: {"array_strategy"}
// Dependencies: {}
pub fn array_strategy < const N : usize > () -> impl Strategy < Value = [String ; N] > { vec (any :: < String > () , N) . prop_map (| v | v . try_into () . unwrap ()) }
};
}

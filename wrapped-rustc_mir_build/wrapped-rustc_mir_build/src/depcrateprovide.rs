// Generated macro for provide (function)
macro_rules! Depcrateprovide {
() => {
// Module: crate
// Provides: {"provide"}
// Dependencies: {}
pub fn provide (providers : & mut Providers) { providers . check_match = thir :: pattern :: check_match ; providers . lit_to_const = thir :: constant :: lit_to_const ; providers . closure_saved_names_of_captured_variables = builder :: closure_saved_names_of_captured_variables ; providers . check_unsafety = check_unsafety :: check_unsafety ; providers . check_tail_calls = check_tail_calls :: check_tail_calls ; providers . thir_body = thir :: cx :: thir_body ; }
};
}

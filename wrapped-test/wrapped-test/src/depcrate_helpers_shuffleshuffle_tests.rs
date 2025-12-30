// Generated macro for shuffle_tests (function)
macro_rules! Depcrate_helpers_shuffleshuffle_tests {
() => {
// Module: crate::helpers::shuffle
// Provides: {"shuffle_tests"}
// Dependencies: {}
pub (crate) fn shuffle_tests (shuffle_seed : u64 , tests : & mut [(TestId , TestDescAndFn)]) { let test_names : Vec < & TestName > = tests . iter () . map (| test | & test . 1 . desc . name) . collect () ; let test_names_hash = calculate_hash (& test_names) ; let mut rng = Rng :: new (shuffle_seed , test_names_hash) ; shuffle (& mut rng , tests) ; }
};
}

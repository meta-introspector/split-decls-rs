// Generated macro for chain_empty_size_hint (function)
macro_rules! Depcrate_io_testschain_empty_size_hint {
() => {
// Module: crate::io::tests
// Provides: {"chain_empty_size_hint"}
// Dependencies: {}
# [test] fn chain_empty_size_hint () { let chain = io :: empty () . chain (io :: empty ()) ; let size_hint = chain . bytes () . size_hint () ; assert_eq ! (size_hint , (0 , Some (0))) ; }
};
}

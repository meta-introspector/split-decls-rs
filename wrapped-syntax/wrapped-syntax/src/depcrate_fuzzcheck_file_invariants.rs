// Generated macro for check_file_invariants (function)
macro_rules! Depcrate_fuzzcheck_file_invariants {
() => {
// Module: crate::fuzz
// Provides: {"check_file_invariants"}
// Dependencies: {}
fn check_file_invariants (file : & SourceFile) { let root = file . syntax () ; validation :: validate_block_structure (root) ; }
};
}

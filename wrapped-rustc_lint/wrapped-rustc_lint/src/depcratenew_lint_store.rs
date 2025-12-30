// Generated macro for new_lint_store (function)
macro_rules! Depcratenew_lint_store {
() => {
// Module: crate
// Provides: {"new_lint_store"}
// Dependencies: {}
pub fn new_lint_store (internal_lints : bool) -> LintStore { let mut lint_store = LintStore :: new () ; register_builtins (& mut lint_store) ; if internal_lints { register_internals (& mut lint_store) ; } lint_store }
};
}

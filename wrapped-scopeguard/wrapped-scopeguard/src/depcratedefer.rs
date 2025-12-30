// Generated macro for defer (macro)
macro_rules! Depcratedefer {
() => {
// Module: crate
// Provides: {"defer"}
// Dependencies: {}
# [doc = " Macro to create a `ScopeGuard` (always run)."] # [doc = ""] # [doc = " The macro takes statements, which are the body of a closure"] # [doc = " that will run when the scope is exited."] # [macro_export] macro_rules ! defer { ($ ($ t : tt) *) => { let _guard = $ crate :: guard (() , | () | { $ ($ t) * }) ; } ; }
};
}

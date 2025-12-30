// Generated macro for defer_on_unwind (macro)
macro_rules! Depcratedefer_on_unwind {
() => {
// Module: crate
// Provides: {"defer_on_unwind"}
// Dependencies: {}
# [doc = " Macro to create a `ScopeGuard` (run on unwinding from panic)."] # [doc = ""] # [doc = " The macro takes statements, which are the body of a closure"] # [doc = " that will run when the scope is exited."] # [doc = ""] # [doc = " Requires crate feature `use_std`."] # [cfg (feature = "use_std")] # [macro_export] macro_rules ! defer_on_unwind { ($ ($ t : tt) *) => { let _guard = $ crate :: guard_on_unwind (() , | () | { $ ($ t) * }) ; } ; }
};
}

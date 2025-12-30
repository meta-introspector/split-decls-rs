// Generated macro for rewrite_macro (function)
macro_rules! Depcrate_macrosrewrite_macro {
() => {
// Module: crate::macros
// Provides: {"rewrite_macro"}
// Dependencies: {}
pub (crate) fn rewrite_macro (mac : & ast :: MacCall , context : & RewriteContext < '_ > , shape : Shape , position : MacroPosition ,) -> RewriteResult { let should_skip = context . skip_context . macros . skip (context . snippet (mac . path . span)) ; if should_skip { Err (RewriteError :: SkipFormatting) } else { let guard = context . enter_macro () ; let result = catch_unwind (AssertUnwindSafe (| | { rewrite_macro_inner (mac , context , shape , position , guard . is_nested ()) })) ; match result { Err (..) => { context . macro_rewrite_failure . replace (true) ; Err (RewriteError :: MacroFailure { kind : MacroErrorKind :: Unknown , span : mac . span () , }) } Ok (Err (e)) => { context . macro_rewrite_failure . replace (true) ; Err (e) } Ok (rw) => rw , } } }
};
}

macro_rules! RustcLintOptDenyFieldAccess {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_deny_field_access)] pub (crate) struct RustcLintOptDenyFieldAccess { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

RustcLintOptDenyFieldAccess!();
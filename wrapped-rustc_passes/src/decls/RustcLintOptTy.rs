macro_rules! RustcLintOptTy {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_ty)] pub (crate) struct RustcLintOptTy { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

RustcLintOptTy!();
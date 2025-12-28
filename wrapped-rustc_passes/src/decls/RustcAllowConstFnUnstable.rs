macro_rules! RustcAllowConstFnUnstable {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_allow_const_fn_unstable)] pub (crate) struct RustcAllowConstFnUnstable { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

RustcAllowConstFnUnstable!()
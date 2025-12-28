macro_rules! RustcForceInlineCoro {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_force_inline_coro)] pub (crate) struct RustcForceInlineCoro { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

RustcForceInlineCoro!()
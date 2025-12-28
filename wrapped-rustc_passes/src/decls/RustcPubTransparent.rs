macro_rules! RustcPubTransparent {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_pub_transparent)] pub (crate) struct RustcPubTransparent { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

RustcPubTransparent!();
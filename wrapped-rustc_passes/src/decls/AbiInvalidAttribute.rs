macro_rules! AbiInvalidAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (passes_abi_invalid_attribute)] pub (crate) struct AbiInvalidAttribute { # [primary_span] pub span : Span , }
    };
}

AbiInvalidAttribute!();
macro_rules! AbiNe {
    () => {
        # [derive (Diagnostic)] # [diag (passes_abi_ne)] pub (crate) struct AbiNe { # [primary_span] pub span : Span , pub left : String , pub right : String , }
    };
}

AbiNe!();
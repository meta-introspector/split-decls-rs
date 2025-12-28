macro_rules! AbiOf {
    () => {
        # [derive (Diagnostic)] # [diag (passes_abi_of)] pub (crate) struct AbiOf { # [primary_span] pub span : Span , pub fn_name : Symbol , pub fn_abi : String , }
    };
}

AbiOf!()
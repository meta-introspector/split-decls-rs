macro_rules! AbiErrorDisabledVectorType {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_abi_error_disabled_vector_type)] # [help] pub (crate) struct AbiErrorDisabledVectorType < 'a > { # [primary_span] # [label] pub span : Span , pub required_feature : & 'a str , pub ty : Ty < 'a > , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }
    };
}

AbiErrorDisabledVectorType!();
macro_rules! RustcLegacyConstGenericsIndexExceed {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_exceed)] pub (crate) struct RustcLegacyConstGenericsIndexExceed { # [primary_span] # [label] pub span : Span , pub arg_count : usize , }
    };
}

RustcLegacyConstGenericsIndexExceed!()
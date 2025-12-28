macro_rules! RustcLegacyConstGenericsIndexNegative {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_negative)] pub (crate) struct RustcLegacyConstGenericsIndexNegative { # [primary_span] pub invalid_args : Vec < Span > , }
    };
}

RustcLegacyConstGenericsIndexNegative!()
macro_rules! RustcLegacyConstGenericsOnly {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_only)] pub (crate) struct RustcLegacyConstGenericsOnly { # [primary_span] pub attr_span : Span , # [label] pub param_span : Span , }
    };
}

RustcLegacyConstGenericsOnly!();
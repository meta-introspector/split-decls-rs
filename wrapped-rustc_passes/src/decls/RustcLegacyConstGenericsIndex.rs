macro_rules! RustcLegacyConstGenericsIndex {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index)] pub (crate) struct RustcLegacyConstGenericsIndex { # [primary_span] pub attr_span : Span , # [label] pub generics_span : Span , }
    };
}

RustcLegacyConstGenericsIndex!()
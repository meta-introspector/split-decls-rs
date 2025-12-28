macro_rules! RustcConstStableIndirectPairing {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_const_stable_indirect_pairing)] pub (crate) struct RustcConstStableIndirectPairing { # [primary_span] pub span : Span , }
    };
}

RustcConstStableIndirectPairing!();
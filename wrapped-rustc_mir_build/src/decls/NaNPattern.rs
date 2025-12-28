macro_rules! NaNPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_nan_pattern)] # [note] # [help] pub (crate) struct NaNPattern { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

NaNPattern!();
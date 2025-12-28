macro_rules! InvalidPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_invalid_pattern)] pub (crate) struct InvalidPattern < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) non_sm_ty : Ty < 'tcx > , pub (crate) prefix : String , }
    };
}

InvalidPattern!()
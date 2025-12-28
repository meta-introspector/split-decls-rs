macro_rules! UnsizedPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_unsized_pattern)] pub (crate) struct UnsizedPattern < 'tcx > { # [primary_span] pub (crate) span : Span , pub (crate) non_sm_ty : Ty < 'tcx > , }
    };
}

UnsizedPattern!();
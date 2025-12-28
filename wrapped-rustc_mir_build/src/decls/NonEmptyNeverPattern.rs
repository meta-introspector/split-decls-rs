macro_rules! NonEmptyNeverPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_non_empty_never_pattern)] # [note] pub (crate) struct NonEmptyNeverPattern < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }
    };
}

NonEmptyNeverPattern!();
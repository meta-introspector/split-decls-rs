macro_rules! TypeNotPartialEq {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_non_partial_eq_match)] # [note (mir_build_type_not_structural_more_info)] pub (crate) struct TypeNotPartialEq < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }
    };
}

TypeNotPartialEq!();
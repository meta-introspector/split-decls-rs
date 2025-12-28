macro_rules! UnionPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_union_pattern)] pub (crate) struct UnionPattern { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

UnionPattern!();
macro_rules! NonConstPath {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_non_const_path , code = E0080)] pub (crate) struct NonConstPath { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

NonConstPath!()
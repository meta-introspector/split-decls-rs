macro_rules! PointerPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_pointer_pattern)] # [note] pub (crate) struct PointerPattern { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

PointerPattern!()
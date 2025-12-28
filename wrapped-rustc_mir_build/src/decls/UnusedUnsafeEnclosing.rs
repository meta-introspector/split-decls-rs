macro_rules! UnusedUnsafeEnclosing {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedUnsafeEnclosing { # [label (mir_build_unused_unsafe_enclosing_block_label)] Block { # [primary_span] span : Span , } , }
    };
}

UnusedUnsafeEnclosing!()
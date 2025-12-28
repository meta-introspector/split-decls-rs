macro_rules! Conflict {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum Conflict { # [label (mir_build_mutable_borrow)] Mut { # [primary_span] span : Span , name : Symbol , } , # [label (mir_build_borrow)] Ref { # [primary_span] span : Span , name : Symbol , } , # [label (mir_build_moved)] Moved { # [primary_span] span : Span , name : Symbol , } , }
    };
}

Conflict!()
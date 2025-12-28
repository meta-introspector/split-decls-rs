macro_rules! MustNotSuspendReason {
    () => {
        # [derive (Subdiagnostic)] # [note (mir_transform_note)] pub (crate) struct MustNotSuspendReason { # [primary_span] pub span : Span , pub reason : String , }
    };
}

MustNotSuspendReason!()
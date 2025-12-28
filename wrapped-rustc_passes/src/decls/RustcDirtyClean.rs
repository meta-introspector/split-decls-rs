macro_rules! RustcDirtyClean {
    () => {
        # [derive (Diagnostic)] # [diag (passes_rustc_dirty_clean)] pub (crate) struct RustcDirtyClean { # [primary_span] pub span : Span , }
    };
}

RustcDirtyClean!();
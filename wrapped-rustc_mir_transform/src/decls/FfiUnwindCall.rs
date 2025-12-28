macro_rules! FfiUnwindCall {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_transform_ffi_unwind_call)] pub (crate) struct FfiUnwindCall { # [label (mir_transform_ffi_unwind_call)] pub span : Span , pub foreign : bool , }
    };
}

FfiUnwindCall!();
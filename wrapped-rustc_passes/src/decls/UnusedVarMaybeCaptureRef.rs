macro_rules! UnusedVarMaybeCaptureRef {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unused_var_maybe_capture_ref)] # [help] pub (crate) struct UnusedVarMaybeCaptureRef { pub name : String , }
    };
}

UnusedVarMaybeCaptureRef!();
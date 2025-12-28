macro_rules! UnusedCaptureMaybeCaptureRef {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unused_capture_maybe_capture_ref)] # [help] pub (crate) struct UnusedCaptureMaybeCaptureRef { pub name : String , }
    };
}

UnusedCaptureMaybeCaptureRef!();
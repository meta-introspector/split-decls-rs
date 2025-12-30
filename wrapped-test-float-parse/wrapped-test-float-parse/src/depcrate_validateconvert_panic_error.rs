// Generated macro for convert_panic_error (function)
macro_rules! Depcrate_validateconvert_panic_error {
() => {
// Module: crate::validate
// Provides: {"convert_panic_error"}
// Dependencies: {}
# [doc = " Turn panics into concrete error types."] fn convert_panic_error (e : & dyn Any , input : & str) -> CheckError { let msg = e . downcast_ref :: < String > () . map (| s | s . as_str ()) . or_else (| | e . downcast_ref :: < & str > () . copied ()) . unwrap_or ("(no contents)") ; CheckError { fail : CheckFailure :: Panic (msg . into ()) , input : input . into () , float_res : "none" . into () , } }
};
}

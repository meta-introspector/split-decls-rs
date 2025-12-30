// Generated macro for reset (function)
macro_rules! Depcrate_deflatereset {
() => {
// Module: crate::deflate
// Provides: {"reset"}
// Dependencies: {}
pub fn reset (stream : & mut DeflateStream) -> ReturnCode { let ret = reset_keep (stream) ; if ret == ReturnCode :: Ok { lm_init (stream . state) ; } ret }
};
}

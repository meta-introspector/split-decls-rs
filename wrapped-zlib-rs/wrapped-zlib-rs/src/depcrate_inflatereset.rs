// Generated macro for reset (function)
macro_rules! Depcrate_inflatereset {
() => {
// Module: crate::inflate
// Provides: {"reset"}
// Dependencies: {}
pub fn reset (stream : & mut InflateStream) -> ReturnCode { stream . state . window . clear () ; stream . state . error_message = None ; reset_keep (stream) }
};
}

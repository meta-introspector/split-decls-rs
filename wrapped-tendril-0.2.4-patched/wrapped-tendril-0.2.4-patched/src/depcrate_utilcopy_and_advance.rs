// Generated macro for copy_and_advance (function)
macro_rules! Depcrate_utilcopy_and_advance {
() => {
// Module: crate::util
// Provides: {"copy_and_advance"}
// Dependencies: {}
# [inline (always)] pub unsafe fn copy_and_advance (dest : & mut * mut u8 , src : & [u8]) { ptr :: copy_nonoverlapping (src . as_ptr () , * dest , src . len ()) ; * dest = dest . offset (src . len () as isize) }
};
}

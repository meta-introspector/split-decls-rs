// Generated macro for set_mode_dict (function)
macro_rules! Depcrate_inflateset_mode_dict {
() => {
// Module: crate::inflate
// Provides: {"set_mode_dict"}
// Dependencies: {}
# [cfg (feature = "__internal-test")] # [doc (hidden)] pub unsafe fn set_mode_dict (strm : & mut z_stream) { unsafe { (* (strm . state as * mut State)) . mode = Mode :: Dict ; } }
};
}

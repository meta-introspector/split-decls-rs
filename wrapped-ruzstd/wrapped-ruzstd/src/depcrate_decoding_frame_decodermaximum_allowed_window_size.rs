// Generated macro for MAXIMUM_ALLOWED_WINDOW_SIZE (const)
macro_rules! Depcrate_decoding_frame_decoderMAXIMUM_ALLOWED_WINDOW_SIZE {
() => {
// Module: crate::decoding::frame_decoder
// Provides: {"MAXIMUM_ALLOWED_WINDOW_SIZE"}
// Dependencies: {}
# [doc = " While the maximum window size allowed by the spec is significantly larger,"] # [doc = " our implementation limits it to 100mb to protect against malformed frames."] const MAXIMUM_ALLOWED_WINDOW_SIZE : u64 = 1024 * 1024 * 100 ;
};
}

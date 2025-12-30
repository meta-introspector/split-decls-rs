// Generated macro for MAX_WINDOW_SIZE (const)
macro_rules! Depcrate_commonMAX_WINDOW_SIZE {
() => {
// Module: crate::common
// Provides: {"MAX_WINDOW_SIZE"}
// Dependencies: {}
# [doc = " Window size refers to the minimum amount of memory needed to decode any given frame."] # [doc = ""] # [doc = " The maximum window size allowed by the spec is 3.75TB"] pub const MAX_WINDOW_SIZE : u64 = (1 << 41) + 7 * (1 << 38) ;
};
}

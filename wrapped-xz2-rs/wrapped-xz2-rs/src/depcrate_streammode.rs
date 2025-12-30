// Generated macro for Mode (enum)
macro_rules! Depcrate_streamMode {
() => {
// Module: crate::stream
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " Compression modes"] # [doc = ""] # [doc = " This selects the function used to analyze the data produced by the match"] # [doc = " finder."] # [derive (Copy , Clone)] pub enum Mode { # [doc = " Fast compression."] # [doc = ""] # [doc = " Fast mode is usually at its best when combined with a hash chain match"] # [doc = " finder."] Fast = lzma_sys :: LZMA_MODE_FAST as isize , # [doc = " Normal compression."] # [doc = ""] # [doc = " This is usually notably slower than fast mode. Use this together with"] # [doc = " binary tree match finders to expose the full potential of the LZMA1 or"] # [doc = " LZMA2 encoder."] Normal = lzma_sys :: LZMA_MODE_NORMAL as isize , }
};
}

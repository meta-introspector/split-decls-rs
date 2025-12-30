// Generated macro for macro_476 (macro)
macro_rules! Depcrate_stream_testsmacro_476 {
() => {
// Module: crate::stream::tests
// Provides: {"macro_476"}
// Dependencies: {}
# [cfg (feature = "std")] proptest ! { # [test] # [cfg_attr (miri , ignore)] fn bit_stream (byte_len in 0 .. 20usize , start in 0 .. 160usize) { bit_stream_inner (byte_len , start) ; } }
};
}

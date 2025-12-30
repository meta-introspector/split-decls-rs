// Generated macro for LineBuffered (struct)
macro_rules! DepcrateLineBuffered {
() => {
// Module: crate
// Provides: {"LineBuffered"}
// Dependencies: {}
# [doc = " A write adapter that buffers writes and automatically flushes on newlines"] pub struct LineBuffered < W , const N : usize > where W : uWrite , { buffer : String < N > , writer : W , }
};
}

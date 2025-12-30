// Generated macro for macro_172 (macro)
macro_rules! Depcrate_iomacro_172 {
() => {
// Module: crate::io
// Provides: {"macro_172"}
// Dependencies: {}
# [cfg (unix)] cfg_aio ! { # [doc = " BSD-specific I/O types."] pub mod bsd { mod poll_aio ; pub use poll_aio :: { Aio , AioEvent , AioSource } ; } }
};
}

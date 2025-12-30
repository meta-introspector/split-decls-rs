// Generated macro for DropFlagMode (enum)
macro_rules! Depcrate_elaborate_dropDropFlagMode {
() => {
// Module: crate::elaborate_drop
// Provides: {"DropFlagMode"}
// Dependencies: {}
# [doc = " Which drop flags to affect/check with an operation."] # [derive (Debug)] pub (crate) enum DropFlagMode { # [doc = " Only affect the top-level drop flag, not that of any contained fields."] Shallow , # [doc = " Affect all nested drop flags in addition to the top-level one."] Deep , }
};
}

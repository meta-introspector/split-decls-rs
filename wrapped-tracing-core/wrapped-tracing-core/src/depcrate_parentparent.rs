// Generated macro for Parent (enum)
macro_rules! Depcrate_parentParent {
() => {
// Module: crate::parent
// Provides: {"Parent"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Parent { # [doc = " The new span will be a root span."] Root , # [doc = " The new span will be rooted in the current span."] Current , # [doc = " The new span has an explicitly-specified parent."] Explicit (Id) , }
};
}

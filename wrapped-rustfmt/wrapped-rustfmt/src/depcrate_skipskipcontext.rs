// Generated macro for SkipContext (struct)
macro_rules! Depcrate_skipSkipContext {
() => {
// Module: crate::skip
// Provides: {"SkipContext"}
// Dependencies: {}
# [doc = " Track which blocks of code are to be skipped when formatting."] # [doc = ""] # [doc = " You can update it by:"] # [doc = ""] # [doc = " - attributes slice"] # [doc = " - manually feeding values into the underlying contexts"] # [doc = ""] # [doc = " Query this context to know if you need to skip a block."] # [derive (Default , Clone)] pub (crate) struct SkipContext { pub (crate) macros : SkipNameContext , pub (crate) attributes : SkipNameContext , }
};
}

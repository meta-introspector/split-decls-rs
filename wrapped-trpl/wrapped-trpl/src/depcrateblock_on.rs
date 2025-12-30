// Generated macro for block_on (function)
macro_rules! Depcrateblock_on {
() => {
// Module: crate
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Run a single future to completion on a bespoke Tokio `Runtime`."] # [doc = ""] # [doc = " Every time you call this, a new instance of `tokio::runtime::Runtime` will"] # [doc = " be created (see the implementation for details: it is trivial). This is:"] # [doc = ""] # [doc = " - Reasonable for teaching purposes, in that you do not generally need to set"] # [doc = "   up more than one runtime anyway, and especially do not in basic code like"] # [doc = "   we are showing!"] # [doc = ""] # [doc = " - Not *that* far off from what Tokio itself does under the hood in its own"] # [doc = "   `tokio::main` macro for supporting `async fn main`."] pub fn block_on < F : Future > (future : F) -> F :: Output { let rt = Runtime :: new () . unwrap () ; rt . block_on (future) }
};
}

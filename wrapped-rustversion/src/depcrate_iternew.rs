// Generated macro for new (function)
macro_rules! Depcrate_iternew {
() => {
// Module: crate::iter
// Provides: {"new"}
// Dependencies: {}
pub fn new (tokens : TokenStream) -> IterImpl { IterImpl { stack : vec ! [tokens . into_iter ()] , peeked : None , } }
};
}

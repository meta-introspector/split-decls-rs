// Generated macro for impl_96 (impl)
macro_rules! Depcrate_errorimpl_96 {
() => {
// Module: crate::error
// Provides: {"impl_96"}
// Dependencies: {}
impl nom :: error :: ContextError < Span < '_ > > for ParseError { fn add_context (input : Span < '_ > , ctx : & 'static str , mut other : Self) -> Self { other . notes . push (ParseErrorNote { location : Location :: from (input) , note : ctx , }) ; other } }
};
}

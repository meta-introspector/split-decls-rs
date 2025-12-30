// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl nom :: error :: ContextError < ParseSpan < '_ > > for ParseError { fn add_context (input : ParseSpan < '_ > , ctx : & 'static str , mut other : Self) -> Self { other . notes . push (ParseErrorNote { location : input . offset () , note : ctx , }) ; other } }
};
}

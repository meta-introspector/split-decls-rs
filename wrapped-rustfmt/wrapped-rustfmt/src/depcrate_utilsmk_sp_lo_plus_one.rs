// Generated macro for mk_sp_lo_plus_one (function)
macro_rules! Depcrate_utilsmk_sp_lo_plus_one {
() => {
// Module: crate::utils
// Provides: {"mk_sp_lo_plus_one"}
// Dependencies: {}
pub (crate) fn mk_sp_lo_plus_one (lo : BytePos) -> Span { Span :: new (lo , lo + BytePos (1) , SyntaxContext :: root () , None) }
};
}

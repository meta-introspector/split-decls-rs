// Generated macro for to_tokens_call (macro)
macro_rules! Depcrate_macrosto_tokens_call {
() => {
// Module: crate::macros
// Provides: {"to_tokens_call"}
// Dependencies: {}
# [cfg (not (feature = "full"))] macro_rules ! to_tokens_call { ($ e : ident , $ tokens : ident , # full $ ($ rest : tt) *) => { unreachable ! () } ; ($ e : ident , $ tokens : ident , $ ($ rest : tt) *) => { $ e . to_tokens ($ tokens) } ; }
};
}

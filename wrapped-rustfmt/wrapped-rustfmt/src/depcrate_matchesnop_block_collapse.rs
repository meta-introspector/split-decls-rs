// Generated macro for nop_block_collapse (function)
macro_rules! Depcrate_matchesnop_block_collapse {
() => {
// Module: crate::matches
// Provides: {"nop_block_collapse"}
// Dependencies: {}
fn nop_block_collapse (block_str : RewriteResult , budget : usize) -> RewriteResult { debug ! ("nop_block_collapse {:?} {}" , block_str , budget) ; block_str . map (| block_str | { if block_str . starts_with ('{') && budget >= 2 && (block_str [1 ..] . find (| c : char | ! c . is_whitespace ()) . unwrap () == block_str . len () - 2) { String :: from ("{}") } else { block_str } }) }
};
}

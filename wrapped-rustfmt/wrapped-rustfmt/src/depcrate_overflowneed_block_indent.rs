// Generated macro for need_block_indent (function)
macro_rules! Depcrate_overflowneed_block_indent {
() => {
// Module: crate::overflow
// Provides: {"need_block_indent"}
// Dependencies: {}
fn need_block_indent (s : & str , shape : Shape) -> bool { s . lines () . skip (1) . any (| s | { s . find (| c | ! char :: is_whitespace (c)) . map_or (false , | w | w + 1 < shape . indent . width ()) }) }
};
}

// Generated macro for StreamSafe (struct)
macro_rules! Depcrate_stream_safeStreamSafe {
() => {
// Module: crate::stream_safe
// Provides: {"StreamSafe"}
// Dependencies: {}
# [doc = " UAX15-D4: This iterator keeps track of how many non-starters there have been"] # [doc = " since the last starter in *NFKD* and will emit a Combining Grapheme Joiner"] # [doc = " (U+034F) if the count exceeds 30."] pub struct StreamSafe < I > { iter : I , nonstarter_count : usize , buffer : Option < char > , }
};
}

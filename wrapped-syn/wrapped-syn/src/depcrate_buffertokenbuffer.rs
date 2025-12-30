// Generated macro for TokenBuffer (struct)
macro_rules! Depcrate_bufferTokenBuffer {
() => {
// Module: crate::buffer
// Provides: {"TokenBuffer"}
// Dependencies: {}
# [doc = " A buffer that can be efficiently traversed multiple times, unlike"] # [doc = " `TokenStream` which requires a deep copy in order to traverse more than"] # [doc = " once."] pub struct TokenBuffer { entries : Box < [Entry] > , }
};
}

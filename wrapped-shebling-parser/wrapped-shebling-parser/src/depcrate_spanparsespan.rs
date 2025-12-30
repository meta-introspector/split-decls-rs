// Generated macro for ParseSpan (struct)
macro_rules! Depcrate_spanParseSpan {
() => {
// Module: crate::span
// Provides: {"ParseSpan"}
// Dependencies: {}
# [doc = " The input to `shebling` parsers."] # [doc = " Besides containing the input source, it also carries the diagnostic context."] # [derive (Clone , Debug)] pub (crate) struct ParseSpan < 'a > { fragment : & 'a str , offset : usize , diags : & 'a ParseDiags , }
};
}

// Generated macro for ABOUT_JAMO_SHORT_NAME (const)
macro_rules! Depcrate_appABOUT_JAMO_SHORT_NAME {
() => {
// Module: crate::app
// Provides: {"ABOUT_JAMO_SHORT_NAME"}
// Dependencies: {}
const ABOUT_JAMO_SHORT_NAME : & 'static str = "\
jamo-short-name parses the UCD's Jamo.txt file and emits its contents as a
slice table. The slice consists of a sorted sequences of pairs, where each
pair corresponds to the codepoint and the Jamo_Short_Name property value.

When emitted as an FST table, the FST corresponds to a map from a Unicode
codepoint (encoded as a big-endian u32) to a u64, where the u64 contains the
Jamo_Short_Name property value. The value is encoded in the least significant
bytes (up to 3).

Since the table is so small, the slice table is faster to search.
" ;
};
}

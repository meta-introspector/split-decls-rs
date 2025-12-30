// Generated macro for impl_139 (impl)
macro_rules! Depcrate_rule_segmenterimpl_139 {
() => {
// Module: crate::rule_segmenter
// Provides: {"impl_139"}
// Dependencies: {}
impl RuleBreakType for Utf16 { type IterAttr < 's > = Utf16Indices < 's > ; type CharType = u32 ; fn char_len (ch : Self :: CharType) -> usize { if ch >= 0x10000 { 2 } else { 1 } } }
};
}

// Generated macro for InlineTableIter (type)
macro_rules! Depcrate_inline_tableInlineTableIter {
() => {
// Module: crate::inline_table
// Provides: {"InlineTableIter"}
// Dependencies: {}
# [doc = " An iterator type over [`InlineTable`]'s [`Key`]/[`Value`] pairs"] pub type InlineTableIter < 'a > = Box < dyn Iterator < Item = (& 'a str , & 'a Value) > + 'a > ;
};
}

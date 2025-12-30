// Generated macro for InlineTableIterMut (type)
macro_rules! Depcrate_inline_tableInlineTableIterMut {
() => {
// Module: crate::inline_table
// Provides: {"InlineTableIterMut"}
// Dependencies: {}
# [doc = " A mutable iterator type over [`InlineTable`]'s [`Key`]/[`Value`] pairs"] pub type InlineTableIterMut < 'a > = Box < dyn Iterator < Item = (KeyMut < 'a > , & 'a mut Value) > + 'a > ;
};
}

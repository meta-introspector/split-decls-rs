// Generated macro for impl_123 (impl)
macro_rules! Depcrate_inline_tableimpl_123 {
() => {
// Module: crate::inline_table
// Provides: {"impl_123"}
// Dependencies: {}
# [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl InlineTable { # [doc = " Creates an empty table."] pub fn new () -> Self { Default :: default () } pub (crate) fn with_pairs (items : KeyValuePairs) -> Self { Self { items , .. Default :: default () } } # [doc = " Convert to a table"] pub fn into_table (self) -> Table { let mut t = Table :: with_pairs (self . items) ; t . fmt () ; t } }
};
}

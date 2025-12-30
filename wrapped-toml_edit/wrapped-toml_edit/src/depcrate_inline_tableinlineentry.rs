// Generated macro for InlineEntry (enum)
macro_rules! Depcrate_inline_tableInlineEntry {
() => {
// Module: crate::inline_table
// Provides: {"InlineEntry"}
// Dependencies: {}
# [doc = " A view into a single location in an [`InlineTable`], which may be vacant or occupied."] pub enum InlineEntry < 'a > { # [doc = " An occupied Entry."] Occupied (InlineOccupiedEntry < 'a >) , # [doc = " A vacant Entry."] Vacant (InlineVacantEntry < 'a >) , }
};
}

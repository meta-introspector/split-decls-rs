// Generated macro for Entry (enum)
macro_rules! Depcrate_tableEntry {
() => {
// Module: crate::table
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single location in a [`Table`], which may be vacant or occupied."] pub enum Entry < 'a > { # [doc = " An occupied Entry."] Occupied (OccupiedEntry < 'a >) , # [doc = " A vacant Entry."] Vacant (VacantEntry < 'a >) , }
};
}

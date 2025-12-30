// Generated macro for Table (struct)
macro_rules! Depcrate_tableTable {
() => {
// Module: crate::table
// Provides: {"Table"}
// Dependencies: {}
pub struct Table { pages : boxcar :: Vec < Page > , # [doc = " Map from ingredient to non-full pages that are up for grabs"] non_full_pages : Mutex < FxHashMap < IngredientIndex , Vec < PageIndex > > > , }
};
}

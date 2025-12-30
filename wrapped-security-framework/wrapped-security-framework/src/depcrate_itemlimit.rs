// Generated macro for Limit (enum)
macro_rules! Depcrate_itemLimit {
() => {
// Module: crate::item
// Provides: {"Limit"}
// Dependencies: {}
# [doc = " Specifies the number of results returned by a search"] # [derive (Debug , Copy , Clone)] pub enum Limit { # [doc = " Always return all results"] All , # [doc = " Return up to the specified number of results"] Max (i64) , }
};
}

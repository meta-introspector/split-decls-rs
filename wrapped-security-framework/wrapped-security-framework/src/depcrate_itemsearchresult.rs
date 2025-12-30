// Generated macro for SearchResult (enum)
macro_rules! Depcrate_itemSearchResult {
() => {
// Module: crate::item
// Provides: {"SearchResult"}
// Dependencies: {}
# [doc = " An individual search result."] pub enum SearchResult { # [doc = " A reference to the Security Framework object, if asked for."] Ref (Reference) , # [doc = " A dictionary of data about the Security Framework object, if asked for."] Dict (CFDictionary) , # [doc = " The Security Framework object as bytes, if asked for."] Data (Vec < u8 >) , # [doc = " An unknown representation of the Security Framework object."] Other , }
};
}

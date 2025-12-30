// Generated macro for ConfigError (enum)
macro_rules! Depcrate_searcherConfigError {
() => {
// Module: crate::searcher
// Provides: {"ConfigError"}
// Dependencies: {}
# [doc = " An error that can occur when building a searcher."] # [doc = ""] # [doc = " This error occurs when a non-sensical configuration is present when trying"] # [doc = " to construct a `Searcher` from a `SearcherBuilder`."] # [derive (Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum ConfigError { # [doc = " Indicates that the heap limit configuration prevents all possible"] # [doc = " search strategies from being used. For example, if the heap limit is"] # [doc = " set to 0 and memory map searching is disabled or unavailable."] SearchUnavailable , # [doc = " Occurs when a matcher reports a line terminator that is different than"] # [doc = " the one configured in the searcher."] MismatchedLineTerminators { # [doc = " The matcher's line terminator."] matcher : LineTerminator , # [doc = " The searcher's line terminator."] searcher : LineTerminator , } , # [doc = " Occurs when no encoding could be found for a particular label."] UnknownEncoding { # [doc = " The provided encoding label that could not be found."] label : Vec < u8 > , } , }
};
}

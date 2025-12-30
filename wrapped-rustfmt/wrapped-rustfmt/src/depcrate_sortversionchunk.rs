// Generated macro for VersionChunk (enum)
macro_rules! Depcrate_sortVersionChunk {
() => {
// Module: crate::sort
// Provides: {"VersionChunk"}
// Dependencies: {}
# [doc = " Represents a chunk in the version-sort algorithm"] # [derive (Debug , PartialEq , Eq)] enum VersionChunk < 'a > { # [doc = " A single `_` in an identifier. Underscores are sorted before all other characters."] Underscore , # [doc = " A &str chunk in the version sort."] Str (& 'a str) , # [doc = " A numeric chunk in the version sort. Keeps track of the numeric value and leading zeros."] Number { value : usize , zeros : usize , source : & 'a str , } , }
};
}

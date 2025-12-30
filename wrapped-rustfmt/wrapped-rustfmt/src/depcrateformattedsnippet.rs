// Generated macro for FormattedSnippet (struct)
macro_rules! DepcrateFormattedSnippet {
() => {
// Module: crate
// Provides: {"FormattedSnippet"}
// Dependencies: {}
# [doc = " Result of formatting a snippet of code along with ranges of lines that didn't get formatted,"] # [doc = " i.e., that got returned as they were originally."] # [derive (Debug)] struct FormattedSnippet { snippet : String , non_formatted_ranges : Vec < (usize , usize) > , }
};
}

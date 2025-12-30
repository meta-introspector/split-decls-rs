// Generated macro for RegexMatcher (struct)
macro_rules! Depcrate_testutilRegexMatcher {
() => {
// Module: crate::testutil
// Provides: {"RegexMatcher"}
// Dependencies: {}
# [doc = " A simple regex matcher."] # [doc = ""] # [doc = " This supports setting the matcher's line terminator configuration directly,"] # [doc = " which we use for testing purposes. That is, the caller explicitly"] # [doc = " determines whether the line terminator optimization is enabled. (In reality"] # [doc = " this optimization is detected automatically by inspecting and possibly"] # [doc = " modifying the regex itself.)"] # [derive (Clone , Debug)] pub (crate) struct RegexMatcher { regex : Regex , line_term : Option < LineTerminator > , every_line_is_candidate : bool , }
};
}

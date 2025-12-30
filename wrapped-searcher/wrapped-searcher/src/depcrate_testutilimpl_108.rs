// Generated macro for impl_108 (impl)
macro_rules! Depcrate_testutilimpl_108 {
() => {
// Module: crate::testutil
// Provides: {"impl_108"}
// Dependencies: {}
impl RegexMatcher { # [doc = " Create a new regex matcher."] pub (crate) fn new (pattern : & str) -> RegexMatcher { let regex = RegexBuilder :: new (pattern) . multi_line (true) . build () . unwrap () ; RegexMatcher { regex , line_term : None , every_line_is_candidate : false } } # [doc = " Forcefully set the line terminator of this matcher."] # [doc = ""] # [doc = " By default, this matcher has no line terminator set."] pub (crate) fn set_line_term (& mut self , line_term : Option < LineTerminator > ,) -> & mut RegexMatcher { self . line_term = line_term ; self } # [doc = " Whether to return every line as a candidate or not."] # [doc = ""] # [doc = " This forces searchers to handle the case of reporting a false positive."] pub (crate) fn every_line_is_candidate (& mut self , yes : bool ,) -> & mut RegexMatcher { self . every_line_is_candidate = yes ; self } }
};
}

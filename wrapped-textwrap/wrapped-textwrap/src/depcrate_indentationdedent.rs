// Generated macro for dedent (function)
macro_rules! Depcrate_indentationdedent {
() => {
// Module: crate::indentation
// Provides: {"dedent"}
// Dependencies: {}
# [doc = " Removes common leading whitespace from each line."] # [doc = ""] # [doc = " This function will look at each non-empty line and determine the"] # [doc = " maximum amount of whitespace that can be removed from all lines:"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::dedent;"] # [doc = ""] # [doc = " assert_eq!(dedent(\""] # [doc = "     1st line"] # [doc = "       2nd line"] # [doc = "     3rd line"] # [doc = " \"), \""] # [doc = " 1st line"] # [doc = "   2nd line"] # [doc = " 3rd line"] # [doc = " \");"] # [doc = " ```"] pub fn dedent (s : & str) -> String { let mut prefix = "" ; let mut lines = s . lines () ; for line in & mut lines { let mut whitespace_idx = line . len () ; for (idx , ch) in line . char_indices () { if ! ch . is_whitespace () { whitespace_idx = idx ; break ; } } if whitespace_idx < line . len () { prefix = & line [.. whitespace_idx] ; break ; } } for line in & mut lines { let mut whitespace_idx = line . len () ; for ((idx , a) , b) in line . char_indices () . zip (prefix . chars ()) { if a != b { whitespace_idx = idx ; break ; } } if whitespace_idx < line . len () && whitespace_idx < prefix . len () { prefix = & line [.. whitespace_idx] ; } } let mut result = String :: new () ; for line in s . lines () { if line . starts_with (prefix) && line . chars () . any (| c | ! c . is_whitespace ()) { let (_ , tail) = line . split_at (prefix . len ()) ; result . push_str (tail) ; } result . push ('\n') ; } if result . ends_with ('\n') && ! s . ends_with ('\n') { let new_len = result . len () - 1 ; result . truncate (new_len) ; } result }
};
}

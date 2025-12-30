// Generated macro for parse_break_test (function)
macro_rules! Depcrate_commonparse_break_test {
() => {
// Module: crate::common
// Provides: {"parse_break_test"}
// Dependencies: {}
# [doc = " A helper function for parsing a single test for the various break"] # [doc = " algorithms."] # [doc = ""] # [doc = " Upon success, this returns the UTF-8 encoded groups of codepoints along"] # [doc = " with the comment associated with the test. The comment is a human readable"] # [doc = " description of the test that may prove useful for debugging."] pub fn parse_break_test (line : & str) -> Result < (Vec < String > , String) , Error > { let re_parts = regex ! (r"(?x)
            ^
            (?:÷|×)
            (?P<groups>(?:\s[0-9A-Fa-f]{4,5}\s(?:÷|×))+)
            \s+
            \#(?P<comment>.+)
            $
            " ,) ; let re_group = regex ! (r"(?x)
            (?P<codepoint>[0-9A-Fa-f]{4,5})\s(?P<kind>÷|×)
            " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid break test line: '{}'" , line) , } ; let comment = caps ["comment"] . trim () . to_string () ; let mut groups = vec ! [] ; let mut cur = String :: new () ; for cap in re_group . captures_iter (& caps ["groups"]) { let cp : Codepoint = cap ["codepoint"] . parse () ? ; let ch = match cp . scalar () { Some (ch) => ch , None => { return err ! ("invalid codepoint '{:X}' in line: '{}'" , cp . value () , line) } } ; cur . push (ch) ; if & cap ["kind"] == "÷" { groups . push (cur) ; cur = String :: new () ; } } Ok ((groups , comment)) }
};
}

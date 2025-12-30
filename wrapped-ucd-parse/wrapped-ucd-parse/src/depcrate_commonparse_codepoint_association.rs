// Generated macro for parse_codepoint_association (function)
macro_rules! Depcrate_commonparse_codepoint_association {
() => {
// Module: crate::common
// Provides: {"parse_codepoint_association"}
// Dependencies: {}
# [doc = " A helper function for parsing a common record format that associates one"] # [doc = " or more codepoints with a string value."] pub fn parse_codepoint_association < 'a > (line : & 'a str ,) -> Result < (Codepoints , & 'a str) , Error > { let re_parts = regex ! (r"(?x)
            ^
            \s*(?P<codepoints>[^\s;]+)\s*;
            \s*(?P<property>[^;\x23]+)\s*
            " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid PropList line: '{}'" , line) , } ; let property = match caps . name ("property") { Some (property) => property . as_str () . trim () , None => { return err ! ("could not find property name in PropList line: '{}'" , line) } } ; Ok ((caps ["codepoints"] . parse () ? , property)) }
};
}

// Generated macro for EastAsianWidth (struct)
macro_rules! Depcrate_east_asian_widthEastAsianWidth {
() => {
// Module: crate::east_asian_width
// Provides: {"EastAsianWidth"}
// Dependencies: {}
# [doc = " A single row in the `EastAsianWidth.txt` file, describing the value of the"] # [doc = " `East_Asian_Width` property."] # [doc = ""] # [doc = " Note: All code points, assigned or unassigned, that are not listed in"] # [doc = " EastAsianWidth.txt file are given the value \"N\"."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct EastAsianWidth { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " One of \"A\", \"F\", \"H\", \"N\", \"Na\", \"W\"."] pub width : String , }
};
}

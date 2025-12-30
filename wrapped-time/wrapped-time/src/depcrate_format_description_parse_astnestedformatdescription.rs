// Generated macro for NestedFormatDescription (struct)
macro_rules! Depcrate_format_description_parse_astNestedFormatDescription {
() => {
// Module: crate::format_description::parse::ast
// Provides: {"NestedFormatDescription"}
// Dependencies: {}
# [doc = " A format description that is nested within another format description."] pub (super) struct NestedFormatDescription < 'a > { # [doc = " Where the opening bracket was in the format string."] pub (super) _opening_bracket : Unused < Location > , # [doc = " The items within the nested format description."] pub (super) items : Box < [Item < 'a >] > , # [doc = " Where the closing bracket was in the format string."] pub (super) _closing_bracket : Unused < Location > , # [doc = " Whitespace between the closing bracket and the next item."] pub (super) _trailing_whitespace : Unused < Option < Spanned < & 'a [u8] > > > , }
};
}

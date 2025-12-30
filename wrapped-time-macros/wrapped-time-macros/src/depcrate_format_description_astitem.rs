// Generated macro for Item (enum)
macro_rules! Depcrate_format_description_astItem {
() => {
// Module: crate::format_description::ast
// Provides: {"Item"}
// Dependencies: {}
pub (super) enum Item < 'a > { Literal (Spanned < & 'a [u8] >) , EscapedBracket { _first : Unused < Location > , _second : Unused < Location > , } , Component { _opening_bracket : Unused < Location > , _leading_whitespace : Unused < Option < Spanned < & 'a [u8] > > > , name : Spanned < & 'a [u8] > , modifiers : Box < [Modifier < 'a >] > , _trailing_whitespace : Unused < Option < Spanned < & 'a [u8] > > > , _closing_bracket : Unused < Location > , } , Optional { opening_bracket : Location , _leading_whitespace : Unused < Option < Spanned < & 'a [u8] > > > , _optional_kw : Unused < Spanned < & 'a [u8] > > , _whitespace : Unused < Spanned < & 'a [u8] > > , nested_format_description : NestedFormatDescription < 'a > , closing_bracket : Location , } , First { opening_bracket : Location , _leading_whitespace : Unused < Option < Spanned < & 'a [u8] > > > , _first_kw : Unused < Spanned < & 'a [u8] > > , _whitespace : Unused < Spanned < & 'a [u8] > > , nested_format_descriptions : Box < [NestedFormatDescription < 'a >] > , closing_bracket : Location , } , }
};
}

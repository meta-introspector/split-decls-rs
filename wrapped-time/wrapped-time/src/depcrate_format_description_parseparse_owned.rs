// Generated macro for parse_owned (function)
macro_rules! Depcrate_format_description_parseparse_owned {
() => {
// Module: crate::format_description::parse
// Provides: {"parse_owned"}
// Dependencies: {}
# [doc = " Parse a sequence of items from the format description."] # [doc = ""] # [doc = " The syntax for the format description can be found in [the"] # [doc = " book](https://time-rs.github.io/book/api/format-description.html). The version of the format"] # [doc = " description is provided as the const parameter."] # [doc = ""] # [doc = " Unlike [`parse`], this function returns [`OwnedFormatItem`], which owns its contents. This means"] # [doc = " that there is no lifetime that needs to be handled. **It is recommended to use version 2.**"] # [doc = ""] # [doc = " [`OwnedFormatItem`]: crate::format_description::OwnedFormatItem"] # [inline] pub fn parse_owned < const VERSION : usize > (s : & str ,) -> Result < format_description :: OwnedFormatItem , error :: InvalidFormatDescription > { validate_version ! (VERSION) ; let mut lexed = lexer :: lex :: < VERSION > (s . as_bytes ()) ; let ast = ast :: parse :: < _ , VERSION > (& mut lexed) ; let format_items = format_item :: parse (ast) ; let items = format_items . collect :: < Result < Box < _ > , _ > > () ? ; Ok (items . into ()) }
};
}

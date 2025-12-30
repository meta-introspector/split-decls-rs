// Generated macro for parse_borrowed (function)
macro_rules! Depcrate_format_description_parseparse_borrowed {
() => {
// Module: crate::format_description::parse
// Provides: {"parse_borrowed"}
// Dependencies: {}
# [doc = " Parse a sequence of items from the format description."] # [doc = ""] # [doc = " The syntax for the format description can be found in [the"] # [doc = " book](https://time-rs.github.io/book/api/format-description.html). The version of the format"] # [doc = " description is provided as the const parameter. **It is recommended to use version 2.**"] # [inline] pub fn parse_borrowed < const VERSION : usize > (s : & str ,) -> Result < Vec < format_description :: BorrowedFormatItem < '_ > > , error :: InvalidFormatDescription > { validate_version ! (VERSION) ; let mut lexed = lexer :: lex :: < VERSION > (s . as_bytes ()) ; let ast = ast :: parse :: < _ , VERSION > (& mut lexed) ; let format_items = format_item :: parse (ast) ; Ok (format_items . map (| res | res . and_then (TryInto :: try_into)) . collect :: < Result < _ , _ > > () ?) }
};
}

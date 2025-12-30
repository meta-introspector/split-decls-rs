// Generated macro for parse_strftime_borrowed (function)
macro_rules! Depcrate_format_description_parse_strftimeparse_strftime_borrowed {
() => {
// Module: crate::format_description::parse::strftime
// Provides: {"parse_strftime_borrowed"}
// Dependencies: {}
# [doc = " Parse a sequence of items from the [`strftime` format description][strftime docs]."] # [doc = ""] # [doc = " The only heap allocation required is for the `Vec` itself. All components are bound to the"] # [doc = " lifetime of the input."] # [doc = ""] # [doc = " [strftime docs]: https://man7.org/linux/man-pages/man3/strftime.3.html"] # [doc (alias = "parse_strptime_borrowed")] # [inline] pub fn parse_strftime_borrowed (s : & str ,) -> Result < Vec < BorrowedFormatItem < '_ > > , InvalidFormatDescription > { let tokens = lex (s . as_bytes ()) ; let items = into_items (tokens) . collect :: < Result < _ , _ > > () ? ; Ok (items) }
};
}

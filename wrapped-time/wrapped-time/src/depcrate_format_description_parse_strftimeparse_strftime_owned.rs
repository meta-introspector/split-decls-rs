// Generated macro for parse_strftime_owned (function)
macro_rules! Depcrate_format_description_parse_strftimeparse_strftime_owned {
() => {
// Module: crate::format_description::parse::strftime
// Provides: {"parse_strftime_owned"}
// Dependencies: {}
# [doc = " Parse a sequence of items from the [`strftime` format description][strftime docs]."] # [doc = ""] # [doc = " This requires heap allocation for some owned items."] # [doc = ""] # [doc = " [strftime docs]: https://man7.org/linux/man-pages/man3/strftime.3.html"] # [doc (alias = "parse_strptime_owned")] # [inline] pub fn parse_strftime_owned (s : & str ,) -> Result < format_description :: OwnedFormatItem , InvalidFormatDescription > { parse_strftime_borrowed (s) . map (Into :: into) }
};
}

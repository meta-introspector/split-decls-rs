// Generated macro for parse (function)
macro_rules! Depcrate_format_description_parseparse {
() => {
// Module: crate::format_description::parse
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse a sequence of items from the format description."] # [doc = ""] # [doc = " The syntax for the format description can be found in [the"] # [doc = " book](https://time-rs.github.io/book/api/format-description.html)."] # [doc = ""] # [doc = " This function exists for backward compatibility reasons. It is equivalent to calling"] # [doc = " `parse_borrowed::<1>(s)`. In the future, this function will be deprecated in favor of"] # [doc = " `parse_borrowed`."] # [inline] pub fn parse (s : & str ,) -> Result < Vec < format_description :: BorrowedFormatItem < '_ > > , error :: InvalidFormatDescription > { parse_borrowed :: < 1 > (s) }
};
}

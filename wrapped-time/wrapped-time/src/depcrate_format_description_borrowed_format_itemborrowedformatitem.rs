// Generated macro for BorrowedFormatItem (enum)
macro_rules! Depcrate_format_description_borrowed_format_itemBorrowedFormatItem {
() => {
// Module: crate::format_description::borrowed_format_item
// Provides: {"BorrowedFormatItem"}
// Dependencies: {}
# [doc = " A complete description of how to format and parse a type."] # [non_exhaustive] # [cfg_attr (not (feature = "alloc") , derive (Debug))] # [derive (Clone , PartialEq , Eq)] pub enum BorrowedFormatItem < 'a > { # [doc = " Bytes that are formatted as-is."] # [doc = ""] # [doc = " **Note**: These bytes **should** be UTF-8, but are not required to be. The value is passed"] # [doc = " through `String::from_utf8_lossy` when necessary."] Literal (& 'a [u8]) , # [doc = " A minimal representation of a single non-literal item."] Component (Component) , # [doc = " A series of literals or components that collectively form a partial or complete"] # [doc = " description."] Compound (& 'a [Self]) , # [doc = " A `FormatItem` that may or may not be present when parsing. If parsing fails, there"] # [doc = " will be no effect on the resulting `struct`."] # [doc = ""] # [doc = " This variant has no effect on formatting, as the value is guaranteed to be present."] Optional (& 'a Self) , # [doc = " A series of `FormatItem`s where, when parsing, the first successful parse is used. When"] # [doc = " formatting, the first element of the slice is used.  An empty slice is a no-op when"] # [doc = " formatting or parsing."] First (& 'a [Self]) , }
};
}

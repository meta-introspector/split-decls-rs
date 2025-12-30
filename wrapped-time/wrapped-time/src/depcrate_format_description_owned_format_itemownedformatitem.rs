// Generated macro for OwnedFormatItem (enum)
macro_rules! Depcrate_format_description_owned_format_itemOwnedFormatItem {
() => {
// Module: crate::format_description::owned_format_item
// Provides: {"OwnedFormatItem"}
// Dependencies: {}
# [doc = " A complete description of how to format and parse a type."] # [non_exhaustive] # [derive (Clone , PartialEq , Eq)] pub enum OwnedFormatItem { # [doc = " Bytes that are formatted as-is."] # [doc = ""] # [doc = " **Note**: These bytes **should** be UTF-8, but are not required to be. The value is passed"] # [doc = " through `String::from_utf8_lossy` when necessary."] Literal (Box < [u8] >) , # [doc = " A minimal representation of a single non-literal item."] Component (Component) , # [doc = " A series of literals or components that collectively form a partial or complete"] # [doc = " description."] Compound (Box < [Self] >) , # [doc = " A `FormatItem` that may or may not be present when parsing. If parsing fails, there"] # [doc = " will be no effect on the resulting `struct`."] # [doc = ""] # [doc = " This variant has no effect on formatting, as the value is guaranteed to be present."] Optional (Box < Self >) , # [doc = " A series of `FormatItem`s where, when parsing, the first successful parse is used. When"] # [doc = " formatting, the first element of the [`Vec`] is used. An empty [`Vec`] is a no-op when"] # [doc = " formatting or parsing."] First (Box < [Self] >) , }
};
}

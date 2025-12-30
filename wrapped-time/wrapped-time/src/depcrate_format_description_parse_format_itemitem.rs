// Generated macro for Item (enum)
macro_rules! Depcrate_format_description_parse_format_itemItem {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"Item"}
// Dependencies: {}
# [doc = " A description of how to format and parse one part of a type."] pub (super) enum Item < 'a > { # [doc = " A literal string."] Literal (& 'a [u8]) , # [doc = " Part of a type, along with its modifiers."] Component (Component) , # [doc = " A sequence of optional items."] Optional { # [doc = " The items themselves."] value : Box < [Self] > , # [doc = " The span of the full sequence."] span : Span , } , # [doc = " The first matching parse of a sequence of format descriptions."] First { # [doc = " The sequence of format descriptions."] value : Box < [Box < [Self] >] > , # [doc = " The span of the full sequence."] span : Span , } , }
};
}

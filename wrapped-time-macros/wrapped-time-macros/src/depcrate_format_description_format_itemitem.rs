// Generated macro for Item (enum)
macro_rules! Depcrate_format_description_format_itemItem {
() => {
// Module: crate::format_description::format_item
// Provides: {"Item"}
// Dependencies: {}
pub (super) enum Item < 'a > { Literal (& 'a [u8]) , Component (Component) , Optional { value : Box < [Self] > , _span : Unused < Span > , } , First { value : Box < [Box < [Self] >] > , _span : Unused < Span > , } , }
};
}

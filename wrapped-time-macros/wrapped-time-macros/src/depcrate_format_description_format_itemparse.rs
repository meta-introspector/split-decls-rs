// Generated macro for parse (function)
macro_rules! Depcrate_format_description_format_itemparse {
() => {
// Module: crate::format_description::format_item
// Provides: {"parse"}
// Dependencies: {}
pub (super) fn parse < 'a > (ast_items : impl Iterator < Item = Result < ast :: Item < 'a > , Error > > ,) -> impl Iterator < Item = Result < Item < 'a > , Error > > { ast_items . map (| ast_item | ast_item . and_then (Item :: from_ast)) }
};
}

// Generated macro for impl_60 (impl)
macro_rules! Depcrate_format_description_format_itemimpl_60 {
() => {
// Module: crate::format_description::format_item
// Provides: {"impl_60"}
// Dependencies: {}
impl Item < '_ > { pub (super) fn from_ast (ast_item : ast :: Item < '_ >) -> Result < Item < '_ > , Error > { Ok (match ast_item { ast :: Item :: Component { _opening_bracket : _ , _leading_whitespace : _ , name , modifiers , _trailing_whitespace : _ , _closing_bracket : _ , } => Item :: Component (component_from_ast (& name , & modifiers) ?) , ast :: Item :: Literal (Spanned { value , span : _ }) => Item :: Literal (value) , ast :: Item :: EscapedBracket { _first : _ , _second : _ , } => Item :: Literal (b"[") , ast :: Item :: Optional { opening_bracket , _leading_whitespace : _ , _optional_kw : _ , _whitespace : _ , nested_format_description , closing_bracket , } => { let items = nested_format_description . items . into_vec () . into_iter () . map (Item :: from_ast) . collect :: < Result < _ , _ > > () ? ; Item :: Optional { value : items , _span : unused (opening_bracket . to (closing_bracket)) , } } ast :: Item :: First { opening_bracket , _leading_whitespace : _ , _first_kw : _ , _whitespace : _ , nested_format_descriptions , closing_bracket , } => { let items = nested_format_descriptions . into_vec () . into_iter () . map (| nested_format_description | { nested_format_description . items . into_vec () . into_iter () . map (Item :: from_ast) . collect () }) . collect :: < Result < _ , _ > > () ? ; Item :: First { value : items , _span : unused (opening_bracket . to (closing_bracket)) , } } }) } }
};
}

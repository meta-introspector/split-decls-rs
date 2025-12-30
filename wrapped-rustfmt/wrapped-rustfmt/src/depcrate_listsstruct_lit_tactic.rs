// Generated macro for struct_lit_tactic (function)
macro_rules! Depcrate_listsstruct_lit_tactic {
() => {
// Module: crate::lists
// Provides: {"struct_lit_tactic"}
// Dependencies: {}
pub (crate) fn struct_lit_tactic (h_shape : Option < Shape > , context : & RewriteContext < '_ > , items : & [ListItem] ,) -> DefinitiveListTactic { if let Some (h_shape) = h_shape { let prelim_tactic = match (context . config . indent_style () , items . len ()) { (IndentStyle :: Visual , 1) => ListTactic :: HorizontalVertical , _ if context . config . struct_lit_single_line () => ListTactic :: HorizontalVertical , _ => ListTactic :: Vertical , } ; definitive_tactic (items , prelim_tactic , Separator :: Comma , h_shape . width) } else { DefinitiveListTactic :: Vertical } }
};
}

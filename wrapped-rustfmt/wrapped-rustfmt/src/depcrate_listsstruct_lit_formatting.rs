// Generated macro for struct_lit_formatting (function)
macro_rules! Depcrate_listsstruct_lit_formatting {
() => {
// Module: crate::lists
// Provides: {"struct_lit_formatting"}
// Dependencies: {}
pub (crate) fn struct_lit_formatting < 'a > (shape : Shape , tactic : DefinitiveListTactic , context : & 'a RewriteContext < '_ > , force_no_trailing_comma : bool ,) -> ListFormatting < 'a > { let ends_with_newline = context . config . indent_style () != IndentStyle :: Visual && tactic == DefinitiveListTactic :: Vertical ; ListFormatting { tactic , separator : "," , trailing_separator : if force_no_trailing_comma { SeparatorTactic :: Never } else { context . config . trailing_comma () } , separator_place : SeparatorPlace :: Back , shape , ends_with_newline , preserve_newline : true , nested : false , align_comments : true , config : context . config , } }
};
}

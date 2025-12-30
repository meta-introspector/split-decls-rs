// Generated macro for macro_539 (macro)
macro_rules! Depcrate_itemmacro_539 {
() => {
// Module: crate::item
// Provides: {"macro_539"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " An item within an impl block."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] # [non_exhaustive] pub enum ImplItem { # [doc = " An associated constant within an impl block."] Const (ImplItemConst) , # [doc = " An associated function within an impl block."] Fn (ImplItemFn) , # [doc = " An associated type within an impl block."] Type (ImplItemType) , # [doc = " A macro invocation within an impl block."] Macro (ImplItemMacro) , # [doc = " Tokens within an impl block not interpreted by Syn."] Verbatim (TokenStream) , } }
};
}

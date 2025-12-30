// Generated macro for macro_947 (macro)
macro_rules! Depcrate_restrictionmacro_947 {
() => {
// Module: crate::restriction
// Provides: {"macro_947"}
// Dependencies: {}
ast_enum ! { # [doc = " The visibility level of an item: inherited or `pub` or"] # [doc = " `pub(restricted)`."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum Visibility { # [doc = " A public visibility level: `pub`."] Public (Token ! [pub]) , # [doc = " A visibility level restricted to some path: `pub(self)` or"] # [doc = " `pub(super)` or `pub(crate)` or `pub(in some::module)`."] Restricted (VisRestricted) , # [doc = " An inherited visibility, which usually means private."] Inherited , } }
};
}

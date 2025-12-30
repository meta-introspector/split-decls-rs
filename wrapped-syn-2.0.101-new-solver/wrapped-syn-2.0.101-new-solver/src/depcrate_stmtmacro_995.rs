// Generated macro for macro_995 (macro)
macro_rules! Depcrate_stmtmacro_995 {
() => {
// Module: crate::stmt
// Provides: {"macro_995"}
// Dependencies: {}
ast_enum ! { # [doc = " A statement, usually ending in a semicolon."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub enum Stmt { # [doc = " A local (let) binding."] Local (Local) , # [doc = " An item definition."] Item (Item) , # [doc = " Expression, with or without trailing semicolon."] Expr (Expr , Option < Token ! [;] >) , # [doc = " A macro invocation in statement position."] # [doc = ""] # [doc = " Syntactically it's ambiguous which other kind of statement this"] # [doc = " macro would expand to. It can be any of local variable (`let`),"] # [doc = " item, or expression."] Macro (StmtMacro) , } }
};
}

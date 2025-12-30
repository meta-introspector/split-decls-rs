// Generated macro for macro_322 (macro)
macro_rules! Depcrate_exprmacro_322 {
() => {
// Module: crate::expr
// Provides: {"macro_322"}
// Dependencies: {}
ast_struct ! { # [doc = " A method call expression: `x.foo::<T>(a, b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprMethodCall { pub attrs : Vec < Attribute >, pub receiver : Box < Expr >, pub dot_token : Token ! [.] , pub method : Ident , pub turbofish : Option < AngleBracketedGenericArguments >, pub paren_token : token :: Paren , pub args : Punctuated < Expr , Token ! [,] >, } }
};
}

// Generated macro for macro_838 (macro)
macro_rules! Depcrate_pathmacro_838 {
() => {
// Module: crate::path
// Provides: {"macro_838"}
// Dependencies: {}
ast_struct ! { # [doc = " Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->"] # [doc = " C`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ParenthesizedGenericArguments { pub paren_token : token :: Paren , # [doc = " `(A, B)`"] pub inputs : Punctuated < Type , Token ! [,] >, # [doc = " `C`"] pub output : ReturnType , } }
};
}

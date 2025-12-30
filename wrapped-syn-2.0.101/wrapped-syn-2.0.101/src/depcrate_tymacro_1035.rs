// Generated macro for macro_1035 (macro)
macro_rules! Depcrate_tymacro_1035 {
() => {
// Module: crate::ty
// Provides: {"macro_1035"}
// Dependencies: {}
ast_struct ! { # [doc = " A bare function type: `fn(usize) -> bool`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeBareFn { pub lifetimes : Option < BoundLifetimes >, pub unsafety : Option < Token ! [unsafe] >, pub abi : Option < Abi >, pub fn_token : Token ! [fn] , pub paren_token : token :: Paren , pub inputs : Punctuated < BareFnArg , Token ! [,] >, pub variadic : Option < BareVariadic >, pub output : ReturnType , } }
};
}

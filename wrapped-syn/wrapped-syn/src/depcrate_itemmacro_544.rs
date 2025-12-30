// Generated macro for macro_544 (macro)
macro_rules! Depcrate_itemmacro_544 {
() => {
// Module: crate::item
// Provides: {"macro_544"}
// Dependencies: {}
ast_struct ! { # [doc = " A function signature in a trait or implementation: `unsafe fn"] # [doc = " initialize(&self)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Signature { pub constness : Option < Token ! [const] >, pub asyncness : Option < Token ! [async] >, pub unsafety : Option < Token ! [unsafe] >, pub abi : Option < Abi >, pub fn_token : Token ! [fn] , pub ident : Ident , pub generics : Generics , pub paren_token : token :: Paren , pub inputs : Punctuated < FnArg , Token ! [,] >, pub variadic : Option < Variadic >, pub output : ReturnType , } }
};
}

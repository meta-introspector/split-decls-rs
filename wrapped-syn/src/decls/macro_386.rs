macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_386 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A function signature in a trait or implementation: `unsafe fn"] # [doc = " initialize(&self)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Signature { pub constness : Option < Token ! [const] >, pub asyncness : Option < Token ! [async] >, pub unsafety : Option < Token ! [unsafe] >, pub abi : Option < Abi >, pub fn_token : Token ! [fn] , pub ident : Ident , pub generics : Generics , pub paren_token : token :: Paren , pub inputs : Punctuated < FnArg , Token ! [,] >, pub variadic : Option < Variadic >, pub output : ReturnType , } }
    };
}

macro_386!();
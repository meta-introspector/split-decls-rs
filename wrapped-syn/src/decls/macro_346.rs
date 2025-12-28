macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_346 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An enum definition: `enum Foo<A, B> { A(A), B(B) }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemEnum { pub attrs : Vec < Attribute >, pub vis : Visibility , pub enum_token : Token ! [enum] , pub ident : Ident , pub generics : Generics , pub brace_token : token :: Brace , pub variants : Punctuated < Variant , Token ! [,] >, } }
    };
}

macro_346!()
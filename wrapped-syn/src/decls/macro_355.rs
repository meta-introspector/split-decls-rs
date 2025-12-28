macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_355 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A trait definition: `pub trait Iterator { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemTrait { pub attrs : Vec < Attribute >, pub vis : Visibility , pub unsafety : Option < Token ! [unsafe] >, pub auto_token : Option < Token ! [auto] >, pub restriction : Option < ImplRestriction >, pub trait_token : Token ! [trait] , pub ident : Ident , pub generics : Generics , pub colon_token : Option < Token ! [:] >, pub supertraits : Punctuated < TypeParamBound , Token ! [+] >, pub brace_token : token :: Brace , pub items : Vec < TraitItem >, } }
    };
}

macro_355!();
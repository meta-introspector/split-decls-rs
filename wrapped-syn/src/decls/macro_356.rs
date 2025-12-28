macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_356 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A trait alias: `pub trait SharableIterator = Iterator + Sync`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemTraitAlias { pub attrs : Vec < Attribute >, pub vis : Visibility , pub trait_token : Token ! [trait] , pub ident : Ident , pub generics : Generics , pub eq_token : Token ! [=] , pub bounds : Punctuated < TypeParamBound , Token ! [+] >, pub semi_token : Token ! [;] , } }
    };
}

macro_356!();
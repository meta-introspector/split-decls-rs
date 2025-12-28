macro_rules! macro_345 {
    () => {
        ast_struct ! { # [doc = " A constant item: `const MAX: u16 = 65535`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemConst { pub attrs : Vec < Attribute >, pub vis : Visibility , pub const_token : Token ! [const] , pub ident : Ident , pub generics : Generics , pub colon_token : Token ! [:] , pub ty : Box < Type >, pub eq_token : Token ! [=] , pub expr : Box < Expr >, pub semi_token : Token ! [;] , } }
    };
}

macro_345!();
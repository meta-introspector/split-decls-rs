macro_rules! macro_353 {
    () => {
        ast_struct ! { # [doc = " A static item: `static BIKE: Shed = Shed(42)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemStatic { pub attrs : Vec < Attribute >, pub vis : Visibility , pub static_token : Token ! [static] , pub mutability : StaticMutability , pub ident : Ident , pub colon_token : Token ! [:] , pub ty : Box < Type >, pub eq_token : Token ! [=] , pub expr : Box < Expr >, pub semi_token : Token ! [;] , } }
    };
}

macro_353!();
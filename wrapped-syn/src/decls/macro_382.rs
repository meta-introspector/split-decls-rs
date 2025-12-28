macro_rules! macro_382 {
    () => {
        ast_struct ! { # [doc = " An associated constant within an impl block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ImplItemConst { pub attrs : Vec < Attribute >, pub vis : Visibility , pub defaultness : Option < Token ! [default] >, pub const_token : Token ! [const] , pub ident : Ident , pub generics : Generics , pub colon_token : Token ! [:] , pub ty : Type , pub eq_token : Token ! [=] , pub expr : Expr , pub semi_token : Token ! [;] , } }
    };
}

macro_382!();
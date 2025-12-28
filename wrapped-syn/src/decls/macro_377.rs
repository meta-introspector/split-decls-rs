macro_rules! macro_377 {
    () => {
        ast_struct ! { # [doc = " An associated constant within the definition of a trait."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct TraitItemConst { pub attrs : Vec < Attribute >, pub const_token : Token ! [const] , pub ident : Ident , pub generics : Generics , pub colon_token : Token ! [:] , pub ty : Type , pub default : Option < (Token ! [=] , Expr) >, pub semi_token : Token ! [;] , } }
    };
}

macro_377!();
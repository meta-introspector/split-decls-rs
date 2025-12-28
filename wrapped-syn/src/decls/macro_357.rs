macro_rules! macro_357 {
    () => {
        ast_struct ! { # [doc = " A type alias: `type Result<T> = std::result::Result<T, MyError>`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemType { pub attrs : Vec < Attribute >, pub vis : Visibility , pub type_token : Token ! [type] , pub ident : Ident , pub generics : Generics , pub eq_token : Token ! [=] , pub ty : Box < Type >, pub semi_token : Token ! [;] , } }
    };
}

macro_357!()
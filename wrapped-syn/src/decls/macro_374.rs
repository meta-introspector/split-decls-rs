macro_rules! macro_374 {
    () => {
        ast_struct ! { # [doc = " A foreign type in an `extern` block: `type void`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ForeignItemType { pub attrs : Vec < Attribute >, pub vis : Visibility , pub type_token : Token ! [type] , pub ident : Ident , pub generics : Generics , pub semi_token : Token ! [;] , } }
    };
}

macro_374!();
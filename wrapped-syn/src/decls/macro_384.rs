macro_rules! macro_384 {
    () => {
        ast_struct ! { # [doc = " An associated type within an impl block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ImplItemType { pub attrs : Vec < Attribute >, pub vis : Visibility , pub defaultness : Option < Token ! [default] >, pub type_token : Token ! [type] , pub ident : Ident , pub generics : Generics , pub eq_token : Token ! [=] , pub ty : Type , pub semi_token : Token ! [;] , } }
    };
}

macro_384!();
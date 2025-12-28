macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_379 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An associated type within the definition of a trait."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct TraitItemType { pub attrs : Vec < Attribute >, pub type_token : Token ! [type] , pub ident : Ident , pub generics : Generics , pub colon_token : Option < Token ! [:] >, pub bounds : Punctuated < TypeParamBound , Token ! [+] >, pub default : Option < (Token ! [=] , Type) >, pub semi_token : Token ! [;] , } }
    };
}

macro_379!()
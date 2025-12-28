macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_293 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A generic type parameter: `T: Into<String>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeParam { pub attrs : Vec < Attribute >, pub ident : Ident , pub colon_token : Option < Token ! [:] >, pub bounds : Punctuated < TypeParamBound , Token ! [+] >, pub eq_token : Option < Token ! [=] >, pub default : Option < Type >, } }
    };
}

macro_293!();
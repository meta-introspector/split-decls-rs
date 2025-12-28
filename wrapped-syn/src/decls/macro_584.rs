macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_584 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An associated type bound: `Iterator<Item: Display>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Constraint { pub ident : Ident , pub generics : Option < AngleBracketedGenericArguments >, pub colon_token : Token ! [:] , pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
    };
}

macro_584!()
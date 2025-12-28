macro_rules! deps {
    () => {
        Lifetime!();
        Punctuated!();
    };
}

macro_rules! macro_292 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A lifetime definition: `'a: 'b + 'c + 'd`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct LifetimeParam { pub attrs : Vec < Attribute >, pub lifetime : Lifetime , pub colon_token : Option < Token ! [:] >, pub bounds : Punctuated < Lifetime , Token ! [+] >, } }
    };
}

macro_292!();
macro_rules! deps {
    () => {
        Punctuated!();
        Lifetime!();
    };
}

macro_rules! macro_328 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A lifetime predicate in a `where` clause: `'a: 'b + 'c`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct PredicateLifetime { pub lifetime : Lifetime , pub colon_token : Token ! [:] , pub bounds : Punctuated < Lifetime , Token ! [+] >, } }
    };
}

macro_328!();
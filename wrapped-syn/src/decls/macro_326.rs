macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_326 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A `where` clause in a definition: `where T: Deserialize<'de>, D:"] # [doc = " 'static`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct WhereClause { pub where_token : Token ! [where] , pub predicates : Punctuated < WherePredicate , Token ! [,] >, } }
    };
}

macro_326!();
macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_581 {
    () => {
        deps!();
        ast_struct ! { # [doc = " Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,"] # [doc = " V>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct AngleBracketedGenericArguments { pub colon2_token : Option < Token ! [::] >, pub lt_token : Token ! [<] , pub args : Punctuated < GenericArgument , Token ! [,] >, pub gt_token : Token ! [>] , } }
    };
}

macro_581!();
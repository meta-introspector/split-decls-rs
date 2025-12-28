macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_317 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A set of bound lifetimes: `for<'a, 'b, 'c>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct BoundLifetimes { pub for_token : Token ! [for] , pub lt_token : Token ! [<] , pub lifetimes : Punctuated < GenericParam , Token ! [,] >, pub gt_token : Token ! [>] , } }
    };
}

macro_317!()
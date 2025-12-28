macro_rules! macro_74 {
    () => {
        ast_struct ! { # [doc = " A name-value pair within an attribute, like `feature = \"nightly\"`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct MetaNameValue { pub path : Path , pub eq_token : Token ! [=] , pub value : Expr , } }
    };
}

macro_74!();
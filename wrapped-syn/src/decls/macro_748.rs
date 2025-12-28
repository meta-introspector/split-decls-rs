macro_rules! macro_748 {
    () => {
        ast_struct ! { # [doc = " Indication that a type should be inferred by the compiler: `_`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeInfer { pub underscore_token : Token ! [_] , } }
    };
}

macro_748!();
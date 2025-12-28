macro_rules! macro_372 {
    () => {
        ast_struct ! { # [doc = " A foreign function in an `extern` block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ForeignItemFn { pub attrs : Vec < Attribute >, pub vis : Visibility , pub sig : Signature , pub semi_token : Token ! [;] , } }
    };
}

macro_372!();
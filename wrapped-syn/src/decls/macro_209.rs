macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_209 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A `continue`, with an optional label."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprContinue # full { pub attrs : Vec < Attribute >, pub continue_token : Token ! [continue] , pub label : Option < Lifetime >, } }
    };
}

macro_209!()
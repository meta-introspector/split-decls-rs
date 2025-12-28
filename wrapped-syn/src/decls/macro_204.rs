macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_204 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A `break`, with an optional label to break and an optional"] # [doc = " expression."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprBreak # full { pub attrs : Vec < Attribute >, pub break_token : Token ! [break] , pub label : Option < Lifetime >, pub expr : Option < Box < Expr >>, } }
    };
}

macro_204!();
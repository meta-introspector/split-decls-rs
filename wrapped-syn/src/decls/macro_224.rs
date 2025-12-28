macro_rules! macro_224 {
    () => {
        ast_struct ! { # [doc = " A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRange # full { pub attrs : Vec < Attribute >, pub start : Option < Box < Expr >>, pub limits : RangeLimits , pub end : Option < Box < Expr >>, } }
    };
}

macro_224!();
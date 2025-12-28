macro_rules! macro_235 {
    () => {
        ast_struct ! { # [doc = " A while loop: `while expr { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprWhile # full { pub attrs : Vec < Attribute >, pub label : Option < Label >, pub while_token : Token ! [while] , pub cond : Box < Expr >, pub body : Block , } }
    };
}

macro_235!();
macro_rules! macro_218 {
    () => {
        ast_struct ! { # [doc = " Conditionless loop: `loop { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprLoop # full { pub attrs : Vec < Attribute >, pub label : Option < Label >, pub loop_token : Token ! [loop] , pub body : Block , } }
    };
}

macro_218!()
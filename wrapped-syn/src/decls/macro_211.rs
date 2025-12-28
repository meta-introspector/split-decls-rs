macro_rules! macro_211 {
    () => {
        ast_struct ! { # [doc = " A for loop: `for pat in expr { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprForLoop # full { pub attrs : Vec < Attribute >, pub label : Option < Label >, pub for_token : Token ! [for] , pub pat : Box < Pat >, pub in_token : Token ! [in] , pub expr : Box < Expr >, pub body : Block , } }
    };
}

macro_211!()
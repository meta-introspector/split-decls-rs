macro_rules! macro_227 {
    () => {
        ast_struct ! { # [doc = " An array literal constructed from one repeated element: `[0u8; N]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRepeat # full { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub expr : Box < Expr >, pub semi_token : Token ! [;] , pub len : Box < Expr >, } }
    };
}

macro_227!();
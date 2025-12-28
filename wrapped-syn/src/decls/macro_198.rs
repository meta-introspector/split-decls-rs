macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_198 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A slice literal expression: `[a, b, c, d]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprArray # full { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Expr , Token ! [,] >, } }
    };
}

macro_198!()
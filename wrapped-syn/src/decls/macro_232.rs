macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_232 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A tuple expression: `(a, b, c, d)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprTuple { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub elems : Punctuated < Expr , Token ! [,] >, } }
    };
}

macro_232!();
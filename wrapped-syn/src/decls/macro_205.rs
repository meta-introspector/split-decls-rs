macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_205 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A function call expression: `invoke(a, b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprCall { pub attrs : Vec < Attribute >, pub func : Box < Expr >, pub paren_token : token :: Paren , pub args : Punctuated < Expr , Token ! [,] >, } }
    };
}

macro_205!()
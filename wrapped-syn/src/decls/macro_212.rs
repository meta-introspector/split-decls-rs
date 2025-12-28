macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! macro_212 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An expression contained within invisible delimiters."] # [doc = ""] # [doc = " This variant is important for faithfully representing the precedence"] # [doc = " of expressions and is related to `None`-delimited spans in a"] # [doc = " `TokenStream`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprGroup { pub attrs : Vec < Attribute >, pub group_token : token :: Group , pub expr : Box < Expr >, } }
    };
}

macro_212!()
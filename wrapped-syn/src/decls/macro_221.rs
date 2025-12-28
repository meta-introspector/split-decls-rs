macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_221 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A method call expression: `x.foo::<T>(a, b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprMethodCall { pub attrs : Vec < Attribute >, pub receiver : Box < Expr >, pub dot_token : Token ! [.] , pub method : Ident , pub turbofish : Option < AngleBracketedGenericArguments >, pub paren_token : token :: Paren , pub args : Punctuated < Expr , Token ! [,] >, } }
    };
}

macro_221!()
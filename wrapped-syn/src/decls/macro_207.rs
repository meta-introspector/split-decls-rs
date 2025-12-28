macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_207 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A closure expression: `|a, b| a + b`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprClosure # full { pub attrs : Vec < Attribute >, pub lifetimes : Option < BoundLifetimes >, pub constness : Option < Token ! [const] >, pub movability : Option < Token ! [static] >, pub asyncness : Option < Token ! [async] >, pub capture : Option < Token ! [move] >, pub or1_token : Token ! [|] , pub inputs : Punctuated < Pat , Token ! [,] >, pub or2_token : Token ! [|] , pub output : ReturnType , pub body : Box < Expr >, } }
    };
}

macro_207!();
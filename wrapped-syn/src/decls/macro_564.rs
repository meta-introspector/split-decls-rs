macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_564 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A tuple pattern: `(a, b)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatTuple { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub elems : Punctuated < Pat , Token ! [,] >, } }
    };
}

macro_564!()
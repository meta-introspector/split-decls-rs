macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_757 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A tuple type: `(A, B, C, String)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeTuple { pub paren_token : token :: Paren , pub elems : Punctuated < Type , Token ! [,] >, } }
    };
}

macro_757!()
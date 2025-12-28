macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_139 {
    () => {
        deps!();
        ast_struct ! { # [doc = " Unnamed fields of a tuple struct or tuple variant such as `Some(T)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct FieldsUnnamed { pub paren_token : token :: Paren , pub unnamed : Punctuated < Field , Token ! [,] >, } }
    };
}

macro_139!();
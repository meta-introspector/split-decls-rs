macro_rules! macro_751 {
    () => {
        ast_struct ! { # [doc = " A parenthesized type equivalent to the inner type."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeParen { pub paren_token : token :: Paren , pub elem : Box < Type >, } }
    };
}

macro_751!()
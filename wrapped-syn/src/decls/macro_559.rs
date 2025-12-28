macro_rules! macro_559 {
    () => {
        ast_struct ! { # [doc = " A parenthesized pattern: `(A | B)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatParen { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub pat : Box < Pat >, } }
    };
}

macro_559!()
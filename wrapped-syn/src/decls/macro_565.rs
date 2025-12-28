macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_565 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatTupleStruct { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , pub paren_token : token :: Paren , pub elems : Punctuated < Pat , Token ! [,] >, } }
    };
}

macro_565!()
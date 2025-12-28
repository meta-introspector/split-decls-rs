macro_rules! macro_223 {
    () => {
        ast_struct ! { # [doc = " A path like `std::mem::replace` possibly containing generic"] # [doc = " parameters and a qualified self-type."] # [doc = ""] # [doc = " A plain identifier like `x` is a path of length 1."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprPath { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , } }
    };
}

macro_223!();
macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_558 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A pattern that matches any one of a set of cases."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatOr { pub attrs : Vec < Attribute >, pub leading_vert : Option < Token ! [|] >, pub cases : Punctuated < Pat , Token ! [|] >, } }
    };
}

macro_558!();
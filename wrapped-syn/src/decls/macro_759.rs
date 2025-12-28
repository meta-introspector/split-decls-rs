macro_rules! macro_759 {
    () => {
        ast_struct ! { # [doc = " An argument in a function type: the `usize` in `fn(usize) -> bool`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct BareFnArg { pub attrs : Vec < Attribute >, pub name : Option < (Ident , Token ! [:]) >, pub ty : Type , } }
    };
}

macro_759!()
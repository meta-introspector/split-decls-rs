macro_rules! macro_760 {
    () => {
        ast_struct ! { # [doc = " The variadic argument of a function pointer like `fn(usize, ...)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct BareVariadic { pub attrs : Vec < Attribute >, pub name : Option < (Ident , Token ! [:]) >, pub dots : Token ! [...] , pub comma : Option < Token ! [,] >, } }
    };
}

macro_760!();
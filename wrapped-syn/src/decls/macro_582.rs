macro_rules! macro_582 {
    () => {
        ast_struct ! { # [doc = " A binding (equality constraint) on an associated type: the `Item = u8`"] # [doc = " in `Iterator<Item = u8>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct AssocType { pub ident : Ident , pub generics : Option < AngleBracketedGenericArguments >, pub eq_token : Token ! [=] , pub ty : Type , } }
    };
}

macro_582!();
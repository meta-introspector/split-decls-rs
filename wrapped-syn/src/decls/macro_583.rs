macro_rules! macro_583 {
    () => {
        ast_struct ! { # [doc = " An equality constraint on an associated constant: the `PANIC = false` in"] # [doc = " `Trait<PANIC = false>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct AssocConst { pub ident : Ident , pub generics : Option < AngleBracketedGenericArguments >, pub eq_token : Token ! [=] , pub value : Expr , } }
    };
}

macro_583!();
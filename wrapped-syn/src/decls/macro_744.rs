macro_rules! macro_744 {
    () => {
        ast_struct ! { # [doc = " A fixed size array type: `[T; n]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeArray { pub bracket_token : token :: Bracket , pub elem : Box < Type >, pub semi_token : Token ! [;] , pub len : Expr , } }
    };
}

macro_744!()
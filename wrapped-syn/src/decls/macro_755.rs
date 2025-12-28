macro_rules! macro_755 {
    () => {
        ast_struct ! { # [doc = " A dynamically sized slice type: `[T]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeSlice { pub bracket_token : token :: Bracket , pub elem : Box < Type >, } }
    };
}

macro_755!();
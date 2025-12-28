macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! macro_746 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A type contained within invisible delimiters."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeGroup { pub group_token : token :: Group , pub elem : Box < Type >, } }
    };
}

macro_746!()
macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_754 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A reference type: `&'a T` or `&'a mut T`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeReference { pub and_token : Token ! [&] , pub lifetime : Option < Lifetime >, pub mutability : Option < Token ! [mut] >, pub elem : Box < Type >, } }
    };
}

macro_754!()
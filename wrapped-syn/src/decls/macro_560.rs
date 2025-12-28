macro_rules! macro_560 {
    () => {
        ast_struct ! { # [doc = " A reference pattern: `&mut var`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatReference { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub mutability : Option < Token ! [mut] >, pub pat : Box < Pat >, } }
    };
}

macro_560!()
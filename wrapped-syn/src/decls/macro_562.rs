macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_562 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatSlice { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Pat , Token ! [,] >, } }
    };
}

macro_562!()
macro_rules! macro_366 {
    () => {
        ast_struct ! { # [doc = " A path prefix of imports in a `use` item: `std::...`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct UsePath { pub ident : Ident , pub colon2_token : Token ! [::] , pub tree : Box < UseTree >, } }
    };
}

macro_366!()
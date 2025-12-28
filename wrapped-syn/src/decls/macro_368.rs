macro_rules! macro_368 {
    () => {
        ast_struct ! { # [doc = " An renamed identifier imported by a `use` item: `HashMap as Map`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct UseRename { pub ident : Ident , pub as_token : Token ! [as] , pub rename : Ident , } }
    };
}

macro_368!()
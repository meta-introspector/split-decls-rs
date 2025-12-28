macro_rules! macro_359 {
    () => {
        ast_struct ! { # [doc = " A use declaration: `use std::collections::HashMap`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemUse { pub attrs : Vec < Attribute >, pub vis : Visibility , pub use_token : Token ! [use] , pub leading_colon : Option < Token ! [::] >, pub tree : UseTree , pub semi_token : Token ! [;] , } }
    };
}

macro_359!()
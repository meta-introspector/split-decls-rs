macro_rules! macro_347 {
    () => {
        ast_struct ! { # [doc = " An `extern crate` item: `extern crate serde`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemExternCrate { pub attrs : Vec < Attribute >, pub vis : Visibility , pub extern_token : Token ! [extern] , pub crate_token : Token ! [crate] , pub ident : Ident , pub rename : Option < (Token ! [as] , Ident) >, pub semi_token : Token ! [;] , } }
    };
}

macro_347!();
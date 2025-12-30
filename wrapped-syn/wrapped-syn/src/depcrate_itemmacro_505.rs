// Generated macro for macro_505 (macro)
macro_rules! Depcrate_itemmacro_505 {
() => {
// Module: crate::item
// Provides: {"macro_505"}
// Dependencies: {}
ast_struct ! { # [doc = " An `extern crate` item: `extern crate serde`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemExternCrate { pub attrs : Vec < Attribute >, pub vis : Visibility , pub extern_token : Token ! [extern] , pub crate_token : Token ! [crate] , pub ident : Ident , pub rename : Option < (Token ! [as] , Ident) >, pub semi_token : Token ! [;] , } }
};
}

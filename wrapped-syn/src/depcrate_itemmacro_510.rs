// Generated macro for macro_510 (macro)
macro_rules! Depcrate_itemmacro_510 {
() => {
// Module: crate::item
// Provides: {"macro_510"}
// Dependencies: {}
ast_struct ! { # [doc = " A module or module declaration: `mod m` or `mod m { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemMod { pub attrs : Vec < Attribute >, pub vis : Visibility , pub unsafety : Option < Token ! [unsafe] >, pub mod_token : Token ! [mod] , pub ident : Ident , pub content : Option < (token :: Brace , Vec < Item >) >, pub semi : Option < Token ! [;] >, } }
};
}

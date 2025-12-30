// Generated macro for macro_525 (macro)
macro_rules! Depcrate_itemmacro_525 {
() => {
// Module: crate::item
// Provides: {"macro_525"}
// Dependencies: {}
ast_struct ! { # [doc = " An associated type within the definition of a trait."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct TraitItemType { pub attrs : Vec < Attribute >, pub type_token : Token ! [type] , pub ident : Ident , pub generics : Generics , pub colon_token : Option < Token ! [:] >, pub bounds : Punctuated < TypeParamBound , Token ! [+] >, pub default : Option < (Token ! [=] , Type) >, pub semi_token : Token ! [;] , } }
};
}

// Generated macro for macro_804 (macro)
macro_rules! Depcrate_patmacro_804 {
() => {
// Module: crate::pat
// Provides: {"macro_804"}
// Dependencies: {}
ast_struct ! { # [doc = " A reference pattern: `&mut var`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatReference { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub mutability : Option < Token ! [mut] >, pub pat : Box < Pat >, } }
};
}

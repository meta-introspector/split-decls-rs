// Generated macro for macro_1014 (macro)
macro_rules! Depcrate_stmtmacro_1014 {
() => {
// Module: crate::stmt
// Provides: {"macro_1014"}
// Dependencies: {}
ast_struct ! { # [doc = " A local `let` binding: `let x: u64 = s.parse()?;`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Local { pub attrs : Vec < Attribute >, pub let_token : Token ! [let] , pub pat : Pat , pub init : Option < LocalInit >, pub semi_token : Token ! [;] , } }
};
}

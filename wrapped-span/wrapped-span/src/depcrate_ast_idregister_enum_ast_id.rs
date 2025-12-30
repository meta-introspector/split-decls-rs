// Generated macro for register_enum_ast_id (macro)
macro_rules! Depcrate_ast_idregister_enum_ast_id {
() => {
// Module: crate::ast_id
// Provides: {"register_enum_ast_id"}
// Dependencies: {}
macro_rules ! register_enum_ast_id { (impl $ AstIdNode : ident for $ ($ ident : ident) ,+) => { $ (impl $ AstIdNode for ast ::$ ident { }) + } ; }
};
}

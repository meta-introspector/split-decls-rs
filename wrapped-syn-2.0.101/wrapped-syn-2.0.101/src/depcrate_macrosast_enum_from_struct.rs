// Generated macro for ast_enum_from_struct (macro)
macro_rules! Depcrate_macrosast_enum_from_struct {
() => {
// Module: crate::macros
// Provides: {"ast_enum_from_struct"}
// Dependencies: {}
macro_rules ! ast_enum_from_struct { ($ name : ident :: Verbatim , $ member : ident) => { } ; ($ name : ident ::$ variant : ident , $ member : ident) => { impl From <$ member > for $ name { fn from (e : $ member) -> $ name { $ name ::$ variant (e) } } } ; }
};
}

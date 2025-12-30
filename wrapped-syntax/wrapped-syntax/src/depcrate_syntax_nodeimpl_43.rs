// Generated macro for impl_43 (impl)
macro_rules! Depcrate_syntax_nodeimpl_43 {
() => {
// Module: crate::syntax_node
// Provides: {"impl_43"}
// Dependencies: {}
impl SyntaxTreeBuilder { pub (crate) fn finish_raw (self) -> (GreenNode , Vec < SyntaxError >) { let green = self . inner . finish () ; (green , self . errors) } pub fn finish (self) -> Parse < SyntaxNode > { let (green , errors) = self . finish_raw () ; # [allow (clippy :: overly_complex_bool_expr)] if cfg ! (debug_assertions) && false { let node = SyntaxNode :: new_root (green . clone ()) ; crate :: validation :: validate_block_structure (& node) ; } Parse :: new (green , errors) } pub fn token (& mut self , kind : SyntaxKind , text : & str) { let kind = RustLanguage :: kind_to_raw (kind) ; self . inner . token (kind , text) ; } pub fn start_node (& mut self , kind : SyntaxKind) { let kind = RustLanguage :: kind_to_raw (kind) ; self . inner . start_node (kind) ; } pub fn finish_node (& mut self) { self . inner . finish_node () ; } pub fn error (& mut self , error : String , text_pos : TextSize) { self . errors . push (SyntaxError :: new_at_offset (error , text_pos)) ; } }
};
}

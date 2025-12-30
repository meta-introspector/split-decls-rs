// Generated macro for UniversalAst (trait)
macro_rules! Depcrate_syn2macroUniversalAst {
() => {
// Module: crate::syn2macro
// Provides: {"UniversalAst"}
// Dependencies: {}
# [doc = " Universal AST execution context trait"] pub trait UniversalAst { type TokenStream ; type Item ; type Error ; fn parse_item (& self , input : Self :: TokenStream) -> Result < Self :: Item , Self :: Error > ; fn transform_item (& self , item : Self :: Item) -> Result < Self :: Item , Self :: Error > ; fn generate_code (& self , item : Self :: Item) -> Self :: TokenStream ; }
};
}

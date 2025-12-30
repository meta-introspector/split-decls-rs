// Generated macro for AstOperation (enum)
macro_rules! Depcrate_syn2macroAstOperation {
() => {
// Module: crate::syn2macro
// Provides: {"AstOperation"}
// Dependencies: {}
# [doc = " AST operation types for security checking"] # [derive (Debug , Clone)] pub enum AstOperation { ParseItem , TransformItem , GenerateCode , FileAccess (String) , NetworkAccess , SystemCall (String) , }
};
}

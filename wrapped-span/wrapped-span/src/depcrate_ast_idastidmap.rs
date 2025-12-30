// Generated macro for AstIdMap (struct)
macro_rules! Depcrate_ast_idAstIdMap {
() => {
// Module: crate::ast_id
// Provides: {"AstIdMap"}
// Dependencies: {}
# [doc = " Maps items' `SyntaxNode`s to `ErasedFileAstId`s and back."] # [derive (Default)] pub struct AstIdMap { # [doc = " An arena of the ptrs and their associated ID."] arena : Arena < (SyntaxNodePtr , ErasedFileAstId) > , # [doc = " Map ptr to id."] ptr_map : hashbrown :: HashTable < ArenaId > , # [doc = " Map id to ptr."] id_map : hashbrown :: HashTable < ArenaId > , }
};
}

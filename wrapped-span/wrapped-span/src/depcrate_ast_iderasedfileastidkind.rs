// Generated macro for ErasedFileAstIdKind (enum)
macro_rules! Depcrate_ast_idErasedFileAstIdKind {
() => {
// Module: crate::ast_id
// Provides: {"ErasedFileAstIdKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Hash , PartialEq , Eq)] # [repr (u8)] enum ErasedFileAstIdKind { # [doc = " This needs to not change because it's depended upon by the proc macro server."] Fixup = 0 , Enum , Struct , Union , ExternCrate , MacroDef , MacroRules , Module , Static , Trait , TraitAlias , Variant , Const , Fn , MacroCall , TypeAlias , ExternBlock , Use , # [doc = " Associated with [`ImplFileAstId`]."] Impl , # [doc = " Associated with [`BlockExprFileAstId`]."] BlockExpr , AsmExpr , # [doc = " Keep this last."] Root , }
};
}

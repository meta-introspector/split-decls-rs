// Generated macro for ErasedAssocItemFileAstId (struct)
macro_rules! Depcrate_ast_idErasedAssocItemFileAstId {
() => {
// Module: crate::ast_id
// Provides: {"ErasedAssocItemFileAstId"}
// Dependencies: {}
# [doc = " This holds the ast ID for variants too (they're a kind of assoc item)."] # [derive (Hash)] struct ErasedAssocItemFileAstId < 'a > { # [doc = " Subtle: items in `extern` blocks **do not** store the ID of the extern block here."] # [doc = " Instead this is left empty. The reason is that `ExternBlockFileAstId` is pretty unstable"] # [doc = " (it contains only an index), and extern blocks don't introduce a new scope, so storing"] # [doc = " the extern block ID will do more harm to incrementality than help."] parent : Option < ErasedFileAstId > , properties : ErasedHasNameFileAstId < 'a > , }
};
}

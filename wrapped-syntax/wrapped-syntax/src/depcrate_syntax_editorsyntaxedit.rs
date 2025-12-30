// Generated macro for SyntaxEdit (struct)
macro_rules! Depcrate_syntax_editorSyntaxEdit {
() => {
// Module: crate::syntax_editor
// Provides: {"SyntaxEdit"}
// Dependencies: {}
# [doc = " Represents a completed [`SyntaxEditor`] operation."] pub struct SyntaxEdit { old_root : SyntaxNode , new_root : SyntaxNode , changed_elements : Vec < SyntaxElement > , annotations : FxHashMap < SyntaxAnnotation , Vec < SyntaxElement > > , }
};
}

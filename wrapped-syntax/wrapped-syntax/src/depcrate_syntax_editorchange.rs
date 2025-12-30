// Generated macro for Change (enum)
macro_rules! Depcrate_syntax_editorChange {
() => {
// Module: crate::syntax_editor
// Provides: {"Change"}
// Dependencies: {}
# [derive (Debug)] enum Change { # [doc = " Inserts a single element at the specified position."] Insert (Position , SyntaxElement) , # [doc = " Inserts many elements in-order at the specified position."] InsertAll (Position , Vec < SyntaxElement >) , # [doc = " Represents both a replace single element and a delete element operation."] Replace (SyntaxElement , Option < SyntaxElement >) , # [doc = " Replaces a single element with many elements."] ReplaceWithMany (SyntaxElement , Vec < SyntaxElement >) , # [doc = " Replaces a range of elements with another list of elements."] # [doc = " Range will always have start != end."] ReplaceAll (RangeInclusive < SyntaxElement > , Vec < SyntaxElement >) , }
};
}

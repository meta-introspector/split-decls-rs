// Generated macro for Converter (struct)
macro_rules! DepcrateConverter {
() => {
// Module: crate
// Provides: {"Converter"}
// Dependencies: {}
struct Converter < SpanMap , S > { current : Option < SyntaxToken > , current_leaves : Vec < tt :: Leaf < S > > , preorder : PreorderWithTokens , range : TextRange , punct_offset : Option < (SyntaxToken , TextSize) > , # [doc = " Used to make the emitted text ranges in the spans relative to the span anchor."] map : SpanMap , append : FxHashMap < SyntaxElement , Vec < tt :: Leaf < S > > > , remove : FxHashSet < SyntaxElement > , call_site : S , mode : DocCommentDesugarMode , }
};
}

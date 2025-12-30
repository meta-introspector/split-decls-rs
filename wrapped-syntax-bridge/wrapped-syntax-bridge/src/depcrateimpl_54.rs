// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < SpanMap , S > Converter < SpanMap , S > { fn new (node : & SyntaxNode , map : SpanMap , append : FxHashMap < SyntaxElement , Vec < tt :: Leaf < S > > > , remove : FxHashSet < SyntaxElement > , call_site : S , mode : DocCommentDesugarMode ,) -> Self { let mut this = Converter { current : None , preorder : node . preorder_with_tokens () , range : node . text_range () , punct_offset : None , map , append , remove , call_site , current_leaves : vec ! [] , mode , } ; let first = this . next_token () ; this . current = first ; this } fn next_token (& mut self) -> Option < SyntaxToken > { while let Some (ev) = self . preorder . next () { match ev { WalkEvent :: Enter (token) => { if self . remove . contains (& token) { match token { syntax :: NodeOrToken :: Token (_) => { continue ; } node => { self . preorder . skip_subtree () ; if let Some (mut v) = self . append . remove (& node) { v . reverse () ; self . current_leaves . extend (v) ; return None ; } } } } else if let syntax :: NodeOrToken :: Token (token) = token { return Some (token) ; } } WalkEvent :: Leave (ele) => { if let Some (mut v) = self . append . remove (& ele) { v . reverse () ; self . current_leaves . extend (v) ; return None ; } } } } None } }
};
}

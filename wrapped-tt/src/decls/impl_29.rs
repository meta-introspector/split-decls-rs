macro_rules! deps {
    () => {
        Delimiter!();
        TokenTreesView!();
        TopSubtreeBuilder!();
        DelimSpan!();
        TopSubtree!();
        Subtree!();
        TokenTree!();
        Leaf!();
        SubtreeView!();
        TtIter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < S : Copy > TopSubtree < S > { pub fn empty (span : DelimSpan < S >) -> Self { Self (Box :: new ([TokenTree :: Subtree (Subtree { delimiter : Delimiter :: invisible_delim_spanned (span) , len : 0 , })])) } pub fn invisible_from_leaves < const N : usize > (delim_span : S , leaves : [Leaf < S > ; N]) -> Self { let mut builder = TopSubtreeBuilder :: new (Delimiter :: invisible_spanned (delim_span)) ; builder . extend (leaves) ; builder . build () } pub fn from_token_trees (delimiter : Delimiter < S > , token_trees : TokenTreesView < '_ , S >) -> Self { let mut builder = TopSubtreeBuilder :: new (delimiter) ; builder . extend_with_tt (token_trees) ; builder . build () } pub fn from_subtree (subtree : SubtreeView < '_ , S >) -> Self { Self (subtree . 0 . into ()) } pub fn view (& self) -> SubtreeView < '_ , S > { SubtreeView :: new (& self . 0) } pub fn iter (& self) -> TtIter < '_ , S > { self . view () . iter () } pub fn top_subtree (& self) -> & Subtree < S > { self . view () . top_subtree () } pub fn top_subtree_delimiter_mut (& mut self) -> & mut Delimiter < S > { let TokenTree :: Subtree (subtree) = & mut self . 0 [0] else { unreachable ! ("the first token tree is always the top subtree") ; } ; & mut subtree . delimiter } pub fn token_trees (& self) -> TokenTreesView < '_ , S > { self . view () . token_trees () } }
    };
}

impl_29!();
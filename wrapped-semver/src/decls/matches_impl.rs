macro_rules! deps {
    () => {
        Version!();
        Op!();
        Comparator!();
    };
}

macro_rules! matches_impl {
    () => {
        deps!();
        fn matches_impl (cmp : & Comparator , ver : & Version) -> bool { match cmp . op { Op :: Exact | Op :: Wildcard => matches_exact (cmp , ver) , Op :: Greater => matches_greater (cmp , ver) , Op :: GreaterEq => matches_exact (cmp , ver) || matches_greater (cmp , ver) , Op :: Less => matches_less (cmp , ver) , Op :: LessEq => matches_exact (cmp , ver) || matches_less (cmp , ver) , Op :: Tilde => matches_tilde (cmp , ver) , Op :: Caret => matches_caret (cmp , ver) , } }
    };
}

matches_impl!()
macro_rules! deps {
    () => {
        ParseStream!();
    };
}

macro_rules! between {
    () => {
        deps!();
        pub (crate) fn between < 'a > (begin : ParseStream < 'a > , end : ParseStream < 'a >) -> TokenStream { let end = end . cursor () ; let mut cursor = begin . cursor () ; assert ! (crate :: buffer :: same_buffer (end , cursor)) ; let mut tokens = TokenStream :: new () ; while cursor != end { let (tt , next) = cursor . token_tree () . unwrap () ; if crate :: buffer :: cmp_assuming_same_buffer (end , next) == Ordering :: Less { if let Some ((inside , _span , after)) = cursor . group (Delimiter :: None) { assert ! (next == after) ; cursor = inside ; continue ; } else { panic ! ("verbatim end must not be inside a delimited group") ; } } tokens . append (tt) ; cursor = next ; } tokens }
    };
}

between!();
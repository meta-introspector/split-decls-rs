macro_rules! deps {
    () => {
        TtIter!();
        Subtree!();
        TokenTree!();
        TtElement!();
        Leaf!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a , S > Iterator for TtIter < 'a , S > { type Item = TtElement < 'a , S > ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () ? { TokenTree :: Leaf (leaf) => Some (TtElement :: Leaf (leaf)) , TokenTree :: Subtree (subtree) => { let nested_iter = TtIter { inner : self . inner . as_slice () [.. subtree . usize_len ()] . iter () } ; self . inner = self . inner . as_slice () [subtree . usize_len () ..] . iter () ; Some (TtElement :: Subtree (subtree , nested_iter)) } } } }
    };
}

impl_13!();
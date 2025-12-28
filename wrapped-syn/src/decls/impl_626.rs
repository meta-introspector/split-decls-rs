macro_rules! deps {
    () => {
        Pair!();
        Punctuated!();
        IntoPairs!();
        End!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl < T , P > Iterator for IntoPairs < T , P > { type Item = Pair < T , P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_626!()
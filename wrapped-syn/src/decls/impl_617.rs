macro_rules! deps {
    () => {
        Punctuated!();
        Pair!();
        Pairs!();
        End!();
    };
}

macro_rules! impl_617 {
    () => {
        deps!();
        impl < 'a , T , P > Iterator for Pairs < 'a , T , P > { type Item = Pair < & 'a T , & 'a P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_617!()
macro_rules! deps {
    () => {
        Punctuated!();
        End!();
        PairsMut!();
        Pair!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < 'a , T , P > Iterator for PairsMut < 'a , T , P > { type Item = Pair < & 'a mut T , & 'a mut P > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (t , p) | Pair :: Punctuated (t , p)) . or_else (| | self . last . next () . map (Pair :: End)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_622!()
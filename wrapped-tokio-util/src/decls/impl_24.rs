macro_rules! deps {
    () => {
        JoinMapKeys!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for JoinMapKeys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < & 'a K > { self . iter . next () . map (| (key , _) | key) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_24!()